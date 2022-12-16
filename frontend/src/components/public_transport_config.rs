use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
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
    let num_connections_to_show = props.config.num_connections_to_show;
    let on_changed_from = move |event: Event| {
        let input = event
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            on_change.emit(PublicTransportConfig {
                base_config: base.clone(),
                from: input.value(),
                to: to.clone(),
                num_connections_to_show,
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
                num_connections_to_show,
            });
        }
    };

    let base = props.config.base_config.clone();
    let on_change = props.on_change.clone();
    let from = props.config.from.clone();
    let to = props.config.to.clone();
    let on_changed_num_connections = move |event: Event| {
        let input = event
            .target()
            .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
        if let Some(input) = input {
            on_change.emit(PublicTransportConfig {
                base_config: base.clone(),
                from: from.clone(),
                to: to.clone(),
                num_connections_to_show: input.value().parse().unwrap_or(1),
            });
        }
    };

    html! {
        <div class="flex flex-col gap-2">
            <label for="from" class="block text-sm font-medium text-slate-300">{"From"}</label>
            <input name="from" type="text" onchange={on_changed_from} value={props.config.from.clone()} placeholder={"Bern, Loryplatz"} class="rounded-sm pl-2 border-slate-300  border-2 bg-transparent text-white"/>
            <label for="to" class="block text-sm font-medium text-slate-300">{"From"}</label>
            <input name="to" type="text" onchange={on_changed_to} value={props.config.to.clone()} placeholder={"Basel"} class="rounded-sm pl-2 border-slate-300 border-2 bg-transparent text-white"/>

            <label for="connections_to_show" class="block text-sm font-medium text-slate-300">{"Number of connections to show"}</label>
            <select name="connections_to_show" onchange={on_changed_num_connections} value={props.config.num_connections_to_show.to_string()} class="w-12 rounded-md p-1">
                {
                    (1..6).into_iter().map(|num_connections| {
                        let is_selected = props.config.num_connections_to_show == num_connections;
                        html!{<option value={num_connections.to_string()} selected={is_selected}>{num_connections}</option>}
                    }).collect::<Html>()
                }
            </select>
        </div>
    }
}
