use anyhow::Error;
use wit_component::ComponentEncoder;

pub struct Loader {}

impl Loader {
    // Since the plugin is built as a wasm module, we use the Component Encoder provided by the `wit_component` crate
    // to encode the wasm module into a component.
    // The preferred way to do this is using the wasm-tools cli, but this method is provided for convenience
    pub fn load_core_module_as_component(binary: &[u8]) -> Result<Vec<u8>, Error> {
        let component = ComponentEncoder::default()
            .module(binary)?
            .validate(true)
            .encode()?;
        Ok(component)
    }

    // pub fn load_component(path: &str) -> Result<Vec<u8>, Error> {
    //     let component =
    //         std::fs::read(path).unwrap_or_else(|_| panic!("No WASM component found at {}", path));
    //     Ok(component)
    // }
}
