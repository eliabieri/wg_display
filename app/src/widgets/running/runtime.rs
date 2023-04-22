use anyhow::Error;
use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};

use crate::widgets::{running::runtime::clocks::Datetime, utils::logging::LOGGING_PREFIX};

use self::types::Host;

wasmtime::component::bindgen!({ path: "src/widgets/wit", world: "plugin" });

pub struct PluginState {}
impl Host for PluginState {}

pub struct Runtime {
    engine: Engine,
    store: Store<PluginState>,
    linker: Linker<PluginState>,
}

impl Runtime {
    pub fn new() -> Self {
        let mut config = Config::new();
        Config::wasm_component_model(&mut config, true);

        let engine = Engine::new(&config).expect("Could not create engine");
        let store = Store::new(&engine, PluginState {});
        let mut linker = Linker::new(&engine);
        Plugin::add_to_linker(&mut linker, |state: &mut PluginState| state)
            .expect("Could not link host API");

        Self {
            engine,
            store,
            linker,
        }
    }

    pub fn instantiate_plugin(&mut self, binary: Vec<u8>) -> Result<Plugin, Error> {
        let start = std::time::Instant::now();
        let component =
            Component::from_binary(&self.engine, &binary).expect("Could not load component");
        let (plugin, _) = Plugin::instantiate(&mut self.store, &component, &self.linker)?;
        let duration = start.elapsed();
        log::info!(
            "{}: Loaded, transformed and compiled module in {} ms",
            LOGGING_PREFIX,
            duration.as_millis()
        );
        Ok(plugin)
    }

    pub fn run_plugin(
        &mut self,
        plugin: &Plugin,
        config_path: &str,
    ) -> wasmtime::Result<PluginResult> {
        let config = std::fs::read(config_path)
            .unwrap_or_else(|_| panic!("No plugin config file at {}", config_path));
        let config = String::from_utf8(config).unwrap();

        let context = PluginContext {
            last_invocation: Datetime {
                seconds: 0,
                nanoseconds: 0,
            },
            config: &config,
        };

        let start = std::time::Instant::now();
        let res = plugin.call_run(&mut self.store, context);
        let duration = start.elapsed();

        log::info!(
            "{}: Plugin invocation took {} ms",
            LOGGING_PREFIX,
            duration.as_millis()
        );
        res
    }
    pub fn get_plugin_name(&mut self, plugin: &Plugin) -> wasmtime::Result<String> {
        plugin.call_get_name(&mut self.store)
    }

    pub fn get_config_schema(&mut self, plugin: &Plugin) -> wasmtime::Result<String> {
        plugin.call_get_config_schema(&mut self.store)
    }
}
