use gloo_net::http::Request;
use yew::{function_component, html, use_effect_with_deps, use_state, AttrValue, Html, Properties};
use yew_hooks::use_clipboard;

use crate::components::divider::DividerComponent;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub widget_name: AttrValue,
}

#[function_component(ConfigSchema)]
pub fn config_schema(props: &Props) -> Html {
    let state = use_state(|| "Loading..".to_string());
    let clipboard = use_clipboard();

    {
        let widget_name = props.widget_name.clone();
        let state_clone = state.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get(format!("/config_schema/{}", widget_name).as_str())
                        .send()
                        .await
                        .expect("Could not load config schema");
                    clipboard.write_text(response.text().await.unwrap());
                    state_clone.set("The schema has been copied to your clipboard.".into());
                });
                || {}
            },
            (),
        );
    }

    let get_title = || format!("Config schema of {}", props.widget_name.as_str());

    html! {
        <div class="bg-zinc-400 h-screen">
            <meta name="viewport" content="width=device-width initial-scale=1.0"/>
            <div class="flex items-center justify-center">
                // Card
                <div class="bg-zinc-200 rounded-2xl p-5 m-10 shadow-2xl">
                    <div>
                        <DividerComponent text={ get_title() }/>
                        <p>{ (*state).clone() }</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
