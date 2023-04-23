use crate::{
    components::{config_card::ConfigCardComponent, divider::DividerComponent},
    routing::router::Route,
};
use common::models::InstallationData;
use gloo_console::log;
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(Install)]
pub fn install() -> Html {
    let installation_data = use_state(InstallationData::default);
    let navigator = use_navigator().unwrap();

    let state = installation_data.clone();
    let on_changed_url = move |event: Event| {
        let input = event
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            let input = input.value();
            state.set(InstallationData {
                download_url: input,
            });
        }
    };

    let on_install_widget = {
        move |_| {
            let installation_data = InstallationData {
                download_url: installation_data.download_url.clone(),
            };
            let navigator_clone = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("/install_widget")
                    .json(&installation_data)
                    .expect("Failed to serialize installation data")
                    .send()
                    .await
                    .expect("Installation data request failed");
                match response.status() {
                    200 => navigator_clone.push(&Route::Home),
                    _ => {
                        log!(
                            "Failed to install widget: {}",
                            response.text().await.unwrap()
                        );
                        navigator_clone.push(&Route::NotFound);
                    }
                };
            })
        }
    };

    html! {
        <div class="bg-zinc-400 h-screen">
            <meta name="viewport" content="width=device-width initial-scale=1.0"/>
            <div class="flex items-center justify-center">
                // Card
                <div class="bg-zinc-200 rounded-2xl p-5 m-10 shadow-2xl">
                    // Flex Container
                    <div class="flex flex-col w-full">
                        // Image
                        <img src="assets/logo.png" alt="" class="h-24 object-contain py-4"/>
                        // Content
                        <div>
                            <DividerComponent text="Installing a widget"/>
                            <ConfigCardComponent>
                                <div>
                                    <label for="url" class="block text-sm font-medium text-slate-300">{"Url"}</label>
                                    <input name="url" type="text" onchange={on_changed_url} class="rounded-sm pl-2 border-slate-300 border-2 bg-transparent text-white"/>
                                    <br/>
                                    <button class="pt-2 font-bold text-slate-300" onclick={on_install_widget}> {"Install"} </button>
                                </div>
                            </ConfigCardComponent>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
