use crate::components::{
    config_card::ConfigCardComponent, divider::DividerComponent, error_display::ErrorDisplay,
};
use common::models::{InstallAction, WidgetStoreItem};
use gloo_console::log;
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlButtonElement, HtmlInputElement};
use yew::prelude::*;
use yew_feather::{Download, Loader};
use yew_router::prelude::*;

async fn load_store_items(
    widget_store_items: UseStateHandle<Vec<WidgetStoreItem>>,
    error: UseStateHandle<Option<String>>,
) {
    let response = Request::get("/get_store_items").send().await;
    if let Err(e) = response {
        error.set(Some(format!("Failed to load widget store items: {}", e)));
        return;
    }

    let items = response.unwrap().json::<Vec<WidgetStoreItem>>().await;
    match items {
        Ok(items) => {
            log!(format!("Loaded widget store items: {items:?}"));
            widget_store_items.set(items);
        }
        Err(e) => {
            error.set(Some(format!(
                "Failed to deserialize widget store items: {}",
                e
            )));
        }
    }
}

async fn install_widget(
    action: InstallAction,
    error: UseStateHandle<Option<String>>,
    is_installing: UseStateHandle<bool>,
    navigator: Navigator,
    widget_store_items: UseStateHandle<Vec<WidgetStoreItem>>,
) {
    is_installing.set(true);
    let response = Request::post("/install_widget")
        .json(&action)
        .expect("Failed to serialize install action")
        .send()
        .await;
    is_installing.set(false);

    match response {
        Err(e) => {
            error.set(Some(format!("Failed to install widget: {}", e)));
            log!("Failed to install widget");
        }
        Ok(response) => match response.status() {
            200 => {
                log!("Successfully installed widget");
                load_store_items(widget_store_items, error).await;
                navigator.push(&crate::Route::Home);
            }
            _ => {
                let response_text = response.text().await;
                let error_text = response_text.unwrap_or("No error message".to_string());
                error.set(Some(error_text));
                log!("Failed to install widget");
            }
        },
    }
}

#[function_component(Install)]
pub fn install() -> Html {
    let installation_data: UseStateHandle<Option<InstallAction>> = use_state(|| None);
    let widget_store_items = use_state(Vec::<WidgetStoreItem>::default);
    let error = use_state(|| None as Option<String>);
    let is_installing = use_state(|| false);
    let navigator = use_navigator().unwrap();

    {
        let widget_store_items = widget_store_items.clone();
        let error = error.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    load_store_items(widget_store_items, error).await;
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
            state.set(Some(InstallAction::FromUrl(input)));
        }
    };

    let on_install_widget_from_url = {
        let error = error.clone();
        let is_installing = is_installing.clone();
        let navigator = navigator.clone();
        let widget_store_items = widget_store_items.clone();
        Callback::from(move |_| {
            if let Some(action) = &*installation_data {
                let error = error.clone();
                let is_installing = is_installing.clone();
                let navigator = navigator.clone();
                let widget_store_items = widget_store_items.clone();
                let action = action.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    install_widget(action, error, is_installing, navigator, widget_store_items).await;
                });
            }
        })
    };

    let on_install_widget = {
        let error = error.clone();
        let is_installing = is_installing.clone();
        let navigator = navigator.clone();
        let widget_store_items = widget_store_items.clone();
        Callback::from(move |event: MouseEvent| {
            let value = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlButtonElement>().ok())
                .map(|btn| btn.value());

            if let Some(value) = value {
                let error = error.clone();
                let is_installing = is_installing.clone();
                let navigator = navigator.clone();
                let widget_store_items = widget_store_items.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    install_widget(
                        InstallAction::FromStoreItemName(value),
                        error,
                        is_installing,
                        navigator,
                        widget_store_items,
                    )
                    .await;
                });
            }
        })
    };

    let on_deinstall_widget = {
        let error = error.clone();
        let is_installing = is_installing.clone();
        let widget_store_items = widget_store_items.clone();
        Callback::from(move |event: MouseEvent| {
            let value = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlButtonElement>().ok())
                .map(|btn| btn.value());

            if let Some(widget_name) = value {
                let error = error.clone();
                let is_installing = is_installing.clone();
                let widget_store_items = widget_store_items.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    is_installing.set(true);
                    let response = Request::get(&format!("/deinstall_widget/{}", widget_name))
                        .send()
                        .await;
                    is_installing.set(false);

                    match response {
                        Err(e) => {
                            error.set(Some(format!("Failed to deinstall widget: {}", e)));
                            log!("Failed to deinstall widget");
                        }
                        Ok(response) => match response.status() {
                            200 => {
                                log!("Successfully deinstalled widget");
                                load_store_items(widget_store_items, error).await;
                            }
                            _ => {
                                let response_text = response.text().await;
                                let error_text = response_text.unwrap_or("No error message".to_string());
                                error.set(Some(error_text));
                                log!("Failed to deinstall widget");
                            }
                        },
                    }
                });
            }
        })
    };

    html! {
        <div class="h-full">
            <meta name="viewport" content="width=device-width initial-scale=1.0"/>
            <div class="flex flex-col items-center justify-center">
                // Card
                <div class="bg-zinc-200 rounded-2xl p-5 m-10 shadow-2xl">
                    // Flex Container
                    <div class="flex flex-col w-full">
                        // Image
                        <img src="assets/logo.png" alt="" class="h-24 object-contain py-4"/>
                        if *is_installing {
                            <div class="flex justify-center items-center pb-4">
                                <Loader class="animate-spin mr-2"/>
                                <span class="text-black text-sm">{"Installing widget..."}</span>
                            </div>
                        }
                        // Content
                        <div>
                            <DividerComponent text="Install from URL"/>
                            <ConfigCardComponent>
                                <div class="flex flex-row justify-between">
                                    <input name="url" type="text" onchange={on_changed_url} class="rounded-sm pl-2 bg-transparent text-white mr-4" placeholder="Url"/>
                                    <button class="text-gray-300 text-sm font-semibold" onclick={on_install_widget_from_url} disabled={*is_installing}>
                                        <Download/>
                                    </button>
                                </div>
                            </ConfigCardComponent>

                            <DividerComponent text="Install from store"/>

                                if widget_store_items.is_empty() {
                                    <p class="text-center text-sm">{"Store items could not be loaded"}</p>
                                }

                                { for widget_store_items.iter().map(|item| {
                                    html! {
                                        <ConfigCardComponent>
                                            <div class="flex flex-col">
                                                <div class="flex flex-row justify-between">
                                                    <div class="flex flex-col pr-4">
                                                        <span class="text-slate-300 text-sm font-semibold"> {&item.name} </span>
                                                        <span class="text-slate-300 text-xs"> {&item.description} </span>
                                                    </div>
                                                    <div class="flex flex-row gap-2">
                                                        <button class="pt-2 text-gray-300 text-sm font-semibold" value={item.name.clone()} onclick={on_deinstall_widget.clone()} disabled={*is_installing}>
                                                            {"Deinstall"}
                                                        </button>
                                                        <button class="pt-2 text-gray-300 text-sm font-semibold" value={item.name.clone()} onclick={on_install_widget.clone()} disabled={*is_installing}>
                                                            {"Install"}
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        </ConfigCardComponent>
                                    }
                                })}
                        </div>
                    </div>
                </div>
                <ErrorDisplay error={error.clone()}></ErrorDisplay>
            </div>
        </div>
    }
}
