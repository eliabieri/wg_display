#[macro_use]
extern crate rocket;

mod configuration_backend;

#[launch]
fn launch() -> _ {
    configuration_backend::serve_dashboard()
}
