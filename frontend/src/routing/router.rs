use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{config_schema::ConfigSchema, home::Home, install::Install};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/install")]
    Install,
    #[at("/config_schema/:widget_name")]
    ConfigSchema { widget_name: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Install => {
            html! {
                <Install />
            }
        }
        Route::ConfigSchema { widget_name } => {
            html! {
                <ConfigSchema widget_name={widget_name} />
            }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
