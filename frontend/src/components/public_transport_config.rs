use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use common::models::PublicTransportConfig;

#[derive(Properties, PartialEq)]
pub struct PublicTransportConfigProps {
    pub config: PublicTransportConfig,
    pub on_change: Callback<PublicTransportConfig>,
}

#[function_component(PublicTransportConfigComponent)]
pub fn public_transport_config_component(props: &PublicTransportConfigProps) -> Html {
    let on_change = props.on_change.clone();
    let base = props.config.base_config.clone();
    let to = props.config.to.clone();
    let on_changed_from = move |event: Event| {
        let input = event
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            on_change.emit(PublicTransportConfig {
                base_config: base.clone(),
                from: input.value(),
                to: to.clone(),
            });
        }
    };

    let base = props.config.base_config.clone();
    let on_change = props.on_change.clone();
    let from = props.config.from.clone();
    let on_changed_to = move |event: Event| {
        let input = event
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            on_change.emit(PublicTransportConfig {
                base_config: base.clone(),
                from: from.clone(),
                to: input.value(),
            });
        }
    };

    html! {
        <div class="flex flex-col gap-2">
            <input type={"text"} onchange={on_changed_from} value={props.config.from.clone()} placeholder={"From"} class="rounded-sm pl-2 border-slate-300  border-2 bg-transparent text-white"/>
            <input type={"text"} onchange={on_changed_to} value={props.config.to.clone()} placeholder={"To"} class="rounded-sm pl-2 border-slate-300 border-2 bg-transparent text-white"/>
        </div>
    }
}
