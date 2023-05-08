//! Frontend for configuring the WG Display.
//
//! It is written in Rust using the Yew framework.
//! It is compiled to WebAssembly and then served by the backend.
//! Amongst others the responsibilities of the frontend are:
//! - Letting users configure which widgets are displayed
//! - Letting users configure certain aspects of the widgets
//! - Letting users configure system aspects like background color

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routing::router::Route;

pub mod components;
mod pages;
mod routing;

#[forbid(unsafe_code)]
#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={routing::router::switch} />
        </BrowserRouter>
    }
}

/// Main entry point for running the frontend in dev mode
/// Use `trunk serve -w` to run it with auto-reload
/// NOTE: Only use this for frontend development, not interaction with the backend is possible with this setup.
fn main() {
    yew::Renderer::<Main>::new().render();
}
