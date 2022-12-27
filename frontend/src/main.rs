//! Frontend for configuring the WG Display.
//
//! It is written in Rust using the Yew framework.
//! It is compiled to WebAssembly and then served by the backend.
//! Amongst others the responsibilities of the frontend are:
//! - Letting users configure which widgets are displayed
//! - Letting users configure certain aspects of the widgets
//! - Letting users configure system aspects like background color

use gloo_console::log;
use gloo_net::http::Request;
use yew::prelude::*;

use common::models::{SystemConfiguration, SystemConfigurationAction};
use common::widget_meta_data::WidgetMetaData;

pub mod components;
use components::default_widget_config::DefaultWidgetConfigComponent;
use components::widget_config::WidgetConfigComponent;

use crate::components::background_color_config::BackgroundColorConfigComponent;
use crate::components::config_card::ConfigCardComponent;
use crate::components::divider::DividerComponent;
use crate::components::public_transport_config::PublicTransportConfigComponent;

#[forbid(unsafe_code)]

/// The main component of the frontend.
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
        <div>
            <meta name="viewport" content="width=device-width initial-scale=1.0"/>
            <div class="flex items-center justify-center bg-zinc-400">
                // Card
                <div class="bg-zinc-200 rounded-2xl p-5 m-10 shadow-2xl">
                    // Flex Container
                    <div class="flex flex-col">
                        // Image
                        <img src="assets/logo.png" alt="" class="h-24 object-contain py-4"/>
                        // Content
                        <div>
                            <DividerComponent text="Configuration"/>

                            <BackgroundColorConfigComponent
                                config={system_config.clone()}
                            />


                            <DividerComponent text="Widget configuration"/>

                            <div>
                                <ConfigCardComponent>
                                    <DefaultWidgetConfigComponent
                                        meta_data={WidgetMetaData::Today}
                                        config={system_config.widget_config.today_config.clone()}
                                    on_change={Callback::from(captures::capture!(clone system_config, |config| {
                                        system_config.dispatch(SystemConfigurationAction::SetTodayConfig(config));
                                    }))}
                                    />
                                </ConfigCardComponent>

                                <ConfigCardComponent>
                                    <DefaultWidgetConfigComponent
                                        meta_data={WidgetMetaData::Aare}
                                        config={system_config.widget_config.aare_config.clone()}
                                    on_change={Callback::from(captures::capture!(clone system_config, |config| {
                                        system_config.dispatch(SystemConfigurationAction::SetAareConfig(config));
                                    }))}
                                    />
                                </ConfigCardComponent>

                                <ConfigCardComponent>
                                    <DefaultWidgetConfigComponent
                                        meta_data={WidgetMetaData::Cafete}
                                        config={system_config.widget_config.cafete_config.clone()}
                                    on_change={Callback::from(captures::capture!(clone system_config, |config| {
                                        system_config.dispatch(SystemConfigurationAction::SetCafeteConfig(config));
                                    }))}
                                    />
                                </ConfigCardComponent>

                                <ConfigCardComponent>
                                    <DefaultWidgetConfigComponent
                                        meta_data={WidgetMetaData::Bernaqua}
                                        config={system_config.widget_config.bernaqua_config.clone()}
                                    on_change={Callback::from(captures::capture!(clone system_config, |config| {
                                        system_config.dispatch(SystemConfigurationAction::SetBernaquaConfig(config));
                                    }))}
                                    />
                                </ConfigCardComponent>

                                <ConfigCardComponent>
                                    <WidgetConfigComponent
                                        meta_data={WidgetMetaData::PublicTransport}
                                        config={system_config.widget_config.public_transport_config.base_config.clone()}
                                        on_change={Callback::from(captures::capture!(clone system_config, |config| {
                                            system_config.dispatch(SystemConfigurationAction::SetPublicTransportBaseConfig(config));
                                        }))}
                                    >
                                        <PublicTransportConfigComponent
                                            config={system_config.widget_config.public_transport_config.clone()}
                                            on_change={Callback::from(captures::capture!(clone system_config, |config| {
                                                system_config.dispatch(SystemConfigurationAction::SetPublicTransportConfig(config));
                                            }))}
                                        />
                                    </WidgetConfigComponent>
                                </ConfigCardComponent>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Main entry point for running the frontend in dev mode
/// Use `trunk serve -w` to run it with auto-reload
/// NOTE: Only use this for frontend development, not interaction with the backend is possible with this setup.
fn main() {
    yew::Renderer::<MainComponent>::new().render();
}
