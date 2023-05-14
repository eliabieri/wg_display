use crate::components::{config_card::ConfigCardComponent, divider::DividerComponent};
use common::models::{InstallationData, WidgetStoreItem};
use gloo_console::log;
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlButtonElement, HtmlInputElement};
use yew::prelude::*;

fn install_widget(installation_data: InstallationData) {
    wasm_bindgen_futures::spawn_local(async move {
        let response = Request::post("/install_widget")
            .json(&installation_data)
            .expect("Failed to serialize installation data")
            .send()
            .await
            .expect("Installation data request failed");
        match response.status() {
            200 => log!(
                "Successfully installed widget: {}",
                response.text().await.unwrap()
            ),
            _ => {
                log!("Failed to install widget");
            }
        };
    })
}

#[function_component(Install)]
pub fn install() -> Html {
    let installation_data: UseStateHandle<Option<InstallationData>> = use_state(|| None);
    let widget_store_items = use_state(Vec::<WidgetStoreItem>::default);

    {
        let widget_store_items = widget_store_items.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("/store_items")
                        .send()
                        .await
                        .expect("Store items request failed");

                    let items = response
                        .json::<Vec<WidgetStoreItem>>()
                        .await
                        .expect("Widget store items could not be deserialized");

                    log!(format!("Loaded widget store items: {items:?}"));
                    widget_store_items.set(items);
                });
            },
            // Run only on first render (no dependencies)
            (),
        );
    }

    let state = installation_data.clone();
    let on_changed_url = move |event: Event| {
        let input = event
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            let input = input.value();
            state.set(Some(InstallationData::DownloadUrl(input)));
        }
    };

    let on_install_widget_from_url = {
        move |_| {
            if installation_data.is_some() {
                install_widget(installation_data.as_ref().unwrap().clone());
            }
        }
    };

    let on_install_widget = move |event: MouseEvent| {
        let value = event
            .target()
            .and_then(|t| t.dyn_into::<HtmlButtonElement>().ok());
        if let Some(value) = value {
            let value = value.value();
            install_widget(InstallationData::Name(value));
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
                            <DividerComponent text="Install from URL"/>
                            <ConfigCardComponent>
                                <div>
                                    <label for="url" class="block text-sm font-medium text-slate-300">{"URL"}</label>
                                    <input name="url" type="text" onchange={on_changed_url} class="rounded-sm pl-2 border-slate-300 border-2 bg-transparent text-white"/>
                                    <br/>
                                    <button class="pt-2 text-gray-300 text-sm font-semibold" onclick={on_install_widget_from_url}> {"Install"} </button>
                                </div>
                            </ConfigCardComponent>

                            <DividerComponent text="Install from store"/>
                            <ConfigCardComponent>
                                <div class="flex flex-col">
                                    { for widget_store_items.iter().map(|item| {
                                        html! {
                                            <div class="flex flex-row justify-between">
                                                <div class="flex flex-col">
                                                    <span class="text-slate-300 text-sm font-semibold"> {&item.name} </span>
                                                    <span class="text-slate-300 text-xs"> {&item.description} </span>
                                                </div>
                                                <button class="pt-2 text-gray-300 text-sm font-semibold" value={item.name.clone()} onclick={on_install_widget}> {"Install"} </button>
                                            </div>
                                        }
                                    })}
                                </div>
                            </ConfigCardComponent>

                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
