use common::models::{SystemConfiguration, SystemConfigurationAction};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::config_card::ConfigCardComponent;

#[derive(Properties, PartialEq)]
pub struct BackgroundColorConfigProps {
    pub config: UseReducerHandle<SystemConfiguration>,
}

#[function_component(BackgroundColorConfigComponent)]
pub fn public_transport_config_component(props: &BackgroundColorConfigProps) -> Html {
    let update_config = {
        let system_config = props.config.clone();
        Callback::from(move |color| {
            system_config.dispatch(SystemConfigurationAction::SetBackgroundColor(color));
        })
    };

    let on_changed = move |event: Event| {
        let input = event
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            update_config.emit(input.value());
        }
    };

    html! {
        <ConfigCardComponent>
            <div class="text-white text-md font-medium">{"Background color"}</div>
            <div class="text-slate-300 text-sm">{"Changes the background color of the display"}</div>
            <input name="background_color" type="color" class="mt-2 bg-transparent" onchange={on_changed} value={props.config.background_color.clone()}/>
        </ConfigCardComponent>
    }
}
