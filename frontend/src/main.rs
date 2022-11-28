use gloo_console::log;
use gloo_net::http::Request;
use yew::prelude::*;

use common::models::{SystemConfiguration, SystemConfigurationAction};
use common::widgets::WidgetName;

pub mod components;
use components::default_widget_config::DefaultWidgetConfigComponent;

#[function_component(MainComponent)]
fn main_component() -> Html {
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
                        "Initialized with system config: {:?}",
                        received_config
                    ));
                    config_clone
                        .dispatch(SystemConfigurationAction::SetInitialConfig(received_config));
                });
                || {}
            },
            (),
        );
    }

    let update_time_config = {
        let system_config = system_config.clone();
        Callback::from(move |time_config| {
            system_config.dispatch(SystemConfigurationAction::SetTimeConfig(time_config));
        })
    };
    let update_aare_config = {
        let system_config = system_config.clone();
        Callback::from(move |aare_config| {
            system_config.dispatch(SystemConfigurationAction::SetAareConfig(aare_config));
        })
    };
    let update_cafete_config = {
        let system_config = system_config.clone();
        Callback::from(move |cafete_config| {
            system_config.dispatch(SystemConfigurationAction::SetCafeteConfig(cafete_config));
        })
    };
    let update_bernaqua_config = {
        let system_config = system_config.clone();
        Callback::from(move |bernaqua_config| {
            system_config.dispatch(SystemConfigurationAction::SetBernaquaConfig(
                bernaqua_config,
            ));
        })
    };
    html! {
        <div class="flex items-center justify-center h-screen bg-zinc-700">
            // Card
            <div class="bg-zinc-800 rounded-2xl p-5 shadow-2xl">
                // Flex Container
                <div class="flex flex-col">
                    // Image
                    <img src="assets/logo.png" alt="" class="h-24 hover:scale-110 duration-500 object-contain"/>
                    // Content
                    <div class="p-3">
                        <p class="text-white leading-5 my-4 text-lg font-bold tracking-wide text-center md:text-left">
                            {"Widget configuration"}
                        </p>

                        <div>
                            <DefaultWidgetConfigComponent
                                widget_name={WidgetName::Time.as_str()}
                                config={system_config.widget_config.time_config.clone()}
                               on_change={update_time_config}
                            />
                            <DefaultWidgetConfigComponent
                                widget_name={WidgetName::Aare.as_str()}
                                config={system_config.widget_config.aare_config.clone()}
                               on_change={update_aare_config}
                            />
                            <DefaultWidgetConfigComponent
                                widget_name={WidgetName::Cafete.as_str()}
                                config={system_config.widget_config.cafete_config.clone()}
                               on_change={update_cafete_config}
                            />
                            <DefaultWidgetConfigComponent
                                widget_name={WidgetName::Bernaqua.as_str()}
                                config={system_config.widget_config.bernaqua_config.clone()}
                               on_change={update_bernaqua_config}
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<MainComponent>::new().render();
}
