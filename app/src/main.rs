//! The WG Display main crate holding everything together.
//! This crate is the entry point for the application.
//! It starts the server to serve the frontend and an API to fetch and modify the configuration.
use futures::join;
use rocket::tokio;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod renderer;
mod server;
pub mod shared;
mod widgets;

#[forbid(unsafe_code)]
#[tokio::main]
async fn main() {
    let _unused = join!(
        tokio::spawn(async { server::serve_dashboard().await }),
        tokio::task::spawn_blocking(|| {
            let mut renderer = renderer::Renderer::new();
            renderer.run();
        })
    );
}
