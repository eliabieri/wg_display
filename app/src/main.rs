#[macro_use]
extern crate rocket;

mod server;

#[launch]
fn launch() -> _ {
    server::serve_dashboard()
}
