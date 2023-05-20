use common::models::{SystemConfiguration, SystemConfigurationAction};
use gloo_console::log;
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, MouseEvent};
use yew::{function_component, html, use_effect_with_deps, use_reducer, use_state, Callback, Html};
use yew_router::prelude::Link;

use crate::components::background_color_config::BackgroundColorConfigComponent;
use crate::components::config_card::ConfigCardComponent;
use crate::components::divider::DividerComponent;
use crate::components::error_display::ErrorDisplay;
use crate::routing::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let system_config = use_reducer(SystemConfiguration::default);
    let error = use_state(|| None as Option<String>);

    {
        // Only runs on first render
        // Initializes the system configuration
        let config_clone = system_config.clone();
        let error = error.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("/config").send().await;
                    let Ok(response) = response else {
                        error.set(Some("Failed to load system config".to_string()));
                        return;
                    };

                    let received_config = response.json::<SystemConfiguration>().await;
                    let Ok(received_config) = received_config else {
                        error.set(Some("Failed to parse system config".to_string()));
                        return;
                    };

                    log!(format!(
                        "Initialized with system config: {received_config:?}"
                    ));
                    config_clone
                        .dispatch(SystemConfigurationAction::SetInitialConfig(received_config));
                });
                || {}
            },
            (),
        );
    }

    let on_deinstall_widget = {
        let error = error.clone();
        Callback::from(move |event: MouseEvent| {
            let event = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlButtonElement>().ok());
            let widget_name = event.map(|e| e.value()).unwrap_or_default();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::get(format!("/deinstall_widget/{}", widget_name).as_str())
                    .send()
                    .await;
                match res {
                    Ok(_) => {
                        log!("Successfully deinstalled widget");
                    }
                    Err(_) => {
                        error.set(Some("Failed to deinstall widget".to_string()));
                    }
                }
            });
        })
    };

    html! {
        <div class="bg-zinc-400 h-screen">
            <meta name="viewport" content="width=device-width initial-scale=1.0"/>
            <div class="flex flex-col items-center justify-center">
                // Card
                <div class="bg-zinc-200 rounded-2xl p-5 m-10 shadow-2xl">
                    // Flex Container
                    <div>
                        // Image
                        <img src="assets/logo.png" alt="" class="h-24 object-contain py-4"/>
                        // Content
                        <div>
                            <DividerComponent text="General"/>

                            <BackgroundColorConfigComponent
                                config={system_config.clone()}
                            />


                            <DividerComponent text="Widgets"/>

                            <div>
                                { for system_config.widgets.iter().map(|widget| {
                                    html! {
                                        <ConfigCardComponent>
                                            <div class="text-white text-lg font-bold pb-2">{widget.name.clone()}</div>
                                            <div class="text-slate-300 text-sm">{widget.description.clone()}</div>
                                            <button value={widget.name.clone()} onclick={on_deinstall_widget.clone()} class="text-gray-300 text-sm font-semibold">{"Deinstall"}</button>
                                            <br/>
                                            <Link<Route> to={Route::ConfigSchema { widget_name: widget.name.clone() }}><div class="text-gray-300 text-sm font-semibold" >{"Copy config schema"}</div></Link<Route>>
                                        </ConfigCardComponent>
                                    }
                                }) }
                            </div>
                            <div>
                                {
                                    if system_config.widgets.is_empty() {
                                        html! {
                                            <p class="text-center text-gray-400">{"No widgets installed"}</p>
                                        }
                                    } else {
                                      html! {}
                                  }
                                }
                            </div>

                            <div class="flex flex-col items-center pt-2">
                                <div type="button" class=" text-zinc-700 border border-zinc-700 hover:bg-zinc-500 hover:text-white active:bg-zinc-500 font-bold text-sm px-3 py-1 rounded outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150">
                                    <Link<Route> to={Route::Install}>{ "Install a widget" }</Link<Route>>
                                </div>
                            </div>

                            <div>
                            </div>
                        </div>
                    </div>
                </div>
                <ErrorDisplay error={error.clone()}></ErrorDisplay>
            </div>
        </div>
    }
}
