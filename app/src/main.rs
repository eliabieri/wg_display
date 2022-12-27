use futures::join;
use rocket::tokio;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod renderer;
mod server;
pub mod shared;

#[forbid(unsafe_code)]

///! The WG Display main crate holding everything together.
#[tokio::main]
async fn main() {
    let mut renderer = renderer::Renderer::new();
    let _unused = join!(
        tokio::spawn(async { server::serve_dashboard().await }),
        renderer.run()
    );
}
