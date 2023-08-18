use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    time::SystemTime,
};

use anyhow::{bail, Error};
use serde::{Deserialize, Serialize};
use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};

use crate::widgets::running::runtime::widget::widget::clocks::Datetime;
use crate::widgets::utils::logging::LOGGING_PREFIX;

wasmtime::component::bindgen!({ path: "../wg_display_widget_wit/wit", world: "widget" });

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CompiledWidget {
    pub data: Vec<u8>,
    compatibility_hash: u64,
}

pub struct WidgetState;

pub struct Runtime {
    engine: Engine,
    store: Store<WidgetState>,
    linker: Linker<WidgetState>,
    last_run: HashMap<String, Datetime>,
}

impl Runtime {
    pub fn new() -> Self {
        let mut config = Config::new();
        Config::wasm_component_model(&mut config, true);

        let engine = Engine::new(&config).expect("Could not create engine");
        let store = Store::new(&engine, WidgetState {});
        let mut linker = Linker::new(&engine);
        Widget::add_to_linker(&mut linker, |state: &mut WidgetState| state)
            .expect("Could not link host API");

        Self {
            engine,
            store,
            linker,
            last_run: HashMap::new(),
        }
    }

    /// Precompile a widget from a binary
    /// The resulting binary can be persisted and later instantiated using `instantiate_widget`
    /// # Arguments
    /// * `bytes` - Binary version of the widget
    /// # Returns
    /// The precompiled widget
    pub fn compile_widget(&self, bytes: &[u8]) -> Result<CompiledWidget, Error> {
        let mut hasher = DefaultHasher::new();
        let compatibility_hash = self.engine.precompile_compatibility_hash();
        compatibility_hash.hash(&mut hasher);
        let compatibility_hash = hasher.finish();

        let data = self.engine.precompile_component(bytes)?;
        Ok(CompiledWidget {
            data,
            compatibility_hash,
        })
    }

    /// Instantiate a widget from a binary that can then be run using `run_widget`
    /// # Arguments
    /// * `widget` - The precompiled widget. Can be produced by `compile_widget`
    /// # Returns
    /// The instantiated widget
    pub fn instantiate_widget(&mut self, widget: &CompiledWidget) -> Result<Widget, Error> {
        // TODO: refactor
        self.last_run.clear();

        if self.needs_recompilation(widget) {
            bail!("Widget needs to be recompiled");
        }
        // Load precompiled component
        // This is only unsafe if the binary is not trusted to come from Engine::precompile_component
        // https://docs.rs/wasmtime/9.0.2/wasmtime/component/struct.Component.html#method.deserialize
        let start = std::time::Instant::now();
        let component = unsafe { Component::deserialize(&self.engine, &widget.data) }?;
        let (widget, _) = Widget::instantiate(&mut self.store, &component, &self.linker)?;
        let duration = start.elapsed();
        log::info!(
            "{}: Deserialized and instantiated widget in {} ms",
            LOGGING_PREFIX,
            duration.as_millis()
        );
        Ok(widget)
    }

    /// Run a widget with a given configuration
    /// # Arguments
    /// * `widget` - The widget to run. Can be produced by `instantiate_widget`
    /// * `config` - The configuration to run the widget with. Must be valid JSON and match the schema returned by `get_config_schema`
    /// # Returns
    /// The result of the widget run
    pub fn run_widget(
        &mut self,
        widget: &Widget,
        config: &str,
    ) -> wasmtime::Result<Option<WidgetResult>> {
        let name = self.get_widget_name(widget).unwrap();
        let last_invocation = self.last_run.get(name.as_str());

        let update_cycle_seconds = widget
            .call_get_run_update_cycle_seconds(&mut self.store)
            .unwrap();
        if let Some(last_invocation) = last_invocation {
            if (last_invocation.seconds + update_cycle_seconds as u64)
                > SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            {
                // Do not updated widget as it is not time yet
                return Ok(None);
            }
        }

        let context = WidgetContext {
            last_invocation: *last_invocation.unwrap_or(&Datetime::now()),
            config: config.to_owned(),
        };

        let start = std::time::Instant::now();
        let res = widget.call_run(&mut self.store, &context);
        let duration = start.elapsed();
        self.last_run.insert(name, Datetime::now());

        log::info!(
            "{}: Widget invocation took {} ms",
            LOGGING_PREFIX,
            duration.as_millis()
        );
        res.map(Some)
    }

    /// Get the name of a widget dynamically
    /// # Arguments
    /// * `widget` - The widget to get the name of. Can be produced by `instantiate_widget`
    /// # Returns
    /// The name of the widget
    pub fn get_widget_name(&mut self, widget: &Widget) -> wasmtime::Result<String> {
        widget.call_get_name(&mut self.store)
    }

    /// Get the configuration schema of a widget
    /// # Arguments
    /// * `widget` - The widget to get the configuration schema of. Can be produced by `instantiate_widget`
    /// # Returns
    /// The configuration schema of the widget as string
    pub fn get_config_schema(&mut self, widget: &Widget) -> wasmtime::Result<String> {
        widget.call_get_config_schema(&mut self.store)
    }

    /// Get the version of a widget
    /// # Arguments
    /// * `widget` - The widget to get the version of. Can be produced by `instantiate_widget`
    /// # Returns
    /// The version of the widget as string
    pub fn get_widget_version(&mut self, widget: &Widget) -> wasmtime::Result<String> {
        widget.call_get_version(&mut self.store)
    }

    /// Check if a widget needs to be recompiled
    /// This can happen if the engine was updated in a non backwards compatible way
    /// # Arguments
    /// * `widget` - The widget to check
    /// # Returns
    /// True if the widget needs to be recompiled
    fn needs_recompilation(&self, widget: &CompiledWidget) -> bool {
        let mut hasher = DefaultHasher::new();
        self.engine
            .precompile_compatibility_hash()
            .hash(&mut hasher);
        let compatibility_hash = hasher.finish();
        compatibility_hash != widget.compatibility_hash
    }
}
