//! The WG Display main crate holding everything together.
//! This crate is the entry point for the application.
//! It starts the server to serve the frontend and an API to fetch and modify the configuration.
use tokio::signal;

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
    // Create a channel for shutdown signaling
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);
    let shutdown_rx_server = shutdown_tx.subscribe();
    let shutdown_rx_renderer = shutdown_tx.subscribe();

    // Spawn a task to handle Ctrl+C
    tokio::spawn(async move {
        if let Err(e) = signal::ctrl_c().await {
            eprintln!("Failed to listen for Ctrl+C: {}", e);
            return;
        }
        println!("\nReceived Ctrl+C, shutting down...");
        let _ = shutdown_tx.send(());
    });

    let server_task = tokio::spawn(async move {
        server::serve_dashboard(shutdown_rx_server).await
    });

    let renderer_task = tokio::task::spawn_blocking(move || {
        let mut renderer = renderer::Renderer::new();
        renderer.run(shutdown_rx_renderer);
    });

    // Wait for either tasks to complete or shutdown signal
    tokio::select! {
        res = server_task => {
            if let Err(e) = res {
                eprintln!("Server task failed: {}", e);
            }
            println!("Server task completed");
        }
        res = renderer_task => {
            if let Err(e) = res {
                eprintln!("Renderer task failed: {}", e);
            }
            println!("Renderer task completed");
        }
        _ = shutdown_rx.recv() => {
            println!("Main received shutdown signal");
        }
    }
}
