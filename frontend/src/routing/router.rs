use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, install::Install};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/install")]
    Install,
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
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
