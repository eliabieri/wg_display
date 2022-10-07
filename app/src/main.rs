use rocket::tokio;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod renderer;
mod server;
pub mod shared;

fn run_app() {
    renderer::Renderer::render();
}

#[launch]
fn launch() -> _ {
    tokio::spawn(async {
        run_app();
    });
    server::serve_dashboard()
}
