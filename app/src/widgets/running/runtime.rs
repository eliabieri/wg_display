use std::collections::HashMap;

use anyhow::Error;
use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};

use crate::widgets::{running::runtime::clocks::Datetime, utils::logging::LOGGING_PREFIX};

use self::types::Host;

wasmtime::component::bindgen!({ path: "../wg_display_widget_wit/wit", world: "widget" });

pub struct WidgetState {}
impl Host for WidgetState {}

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

    pub fn instantiate_widget(&mut self, binary: &[u8]) -> Result<Widget, Error> {
        let start = std::time::Instant::now();
        let component =
            Component::from_binary(&self.engine, binary).expect("Could not load component");
        let (widget, _) = Widget::instantiate(&mut self.store, &component, &self.linker)?;
        let duration = start.elapsed();
        log::info!(
            "{}: Loaded, transformed and compiled component in {} ms",
            LOGGING_PREFIX,
            duration.as_millis()
        );
        Ok(widget)
    }

    pub fn run_widget(&mut self, widget: &Widget, config: &str) -> wasmtime::Result<WidgetResult> {
        let name = self.get_widget_name(widget)?;
        let last_invocation = *self.last_run.get(&name).unwrap_or(&Datetime::now());
        let context = WidgetContext {
            last_invocation,
            config,
        };

        let start = std::time::Instant::now();
        let res = widget.call_run(&mut self.store, context);
        let duration = start.elapsed();

        log::info!(
            "{}: Widget invocation took {} ms",
            LOGGING_PREFIX,
            duration.as_millis()
        );
        res
    }
    pub fn get_widget_name(&mut self, widget: &Widget) -> wasmtime::Result<String> {
        widget.call_get_name(&mut self.store)
    }

    pub fn get_config_schema(&mut self, widget: &Widget) -> wasmtime::Result<String> {
        widget.call_get_config_schema(&mut self.store)
    }
}
