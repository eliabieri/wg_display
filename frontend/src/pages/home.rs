use common::models::{SystemConfiguration, SystemConfigurationAction};
use gloo_console::log;
use gloo_net::http::Request;
use yew::{function_component, html, use_effect_with_deps, use_reducer, Html};
use yew_router::prelude::Link;

use crate::components::background_color_config::BackgroundColorConfigComponent;
use crate::components::config_card::ConfigCardComponent;
use crate::components::divider::DividerComponent;
use crate::routing::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let system_config = use_reducer(SystemConfiguration::default);

    {
        // Only runs on first render
        // Initializes the system configuration
        let config_clone = system_config.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("/config")
                        .send()
                        .await
                        .expect("Config request failed");

                    let received_config = response
                        .json::<SystemConfiguration>()
                        .await
                        .expect("Recevied config could not be deserialized");

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

    html! {
        <div class="bg-zinc-400 h-screen">
            <meta name="viewport" content="width=device-width initial-scale=1.0"/>
            <div class="flex items-center justify-center">
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
                                { for system_config.widget_config.iter().map(|widget| {
                                    html! {
                                        <ConfigCardComponent>
                                            <div class="text-white text-md font-medium">{widget.name.clone()}</div>
                                            <div class="text-slate-300 text-sm">{"Currently, widgets cannot be deinstalled"}</div>
                                        </ConfigCardComponent>
                                    }
                                }) }
                            </div>
                            <div>
                                {
                                    if system_config.widget_config.is_empty() {
                                        html! {
                                            <p class="text-center text-gray-400">{"No widgets installed"}</p>
                                        }
                                    } else {
                                      html! {}
                                  }
                                }
                            </div>

                            <div class="flex flex-col items-center pt-2">
                                <div type="button" class=" text-zinc-700 border border-zinc-700 hover:bg-zinc-500 hover:text-white active:bg-zinc-500 font-bold text-sm px-3 py-1 uppercase rounded outline-none focus:outline-none mr-1 mb-1 ease-linear transition-all duration-150 w-min">
                                    <Link<Route> to={Route::Install}>{ "Add" }</Link<Route>>
                                </div>
                            </div>

                            <div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
