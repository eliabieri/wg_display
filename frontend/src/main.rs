use gloo_console::log;
use gloo_net::http::Request;
use yew::prelude::*;

use common::models::{SystemConfiguration, SystemConfigurationAction};
use common::widget_meta_data::WidgetMetaData;

pub mod components;
use components::default_widget_config::DefaultWidgetConfigComponent;
use components::public_transport_config::PublicTransportConfigComponent;
use components::widget_config::WidgetConfigComponent;

use crate::components::config_card::ConfigCardComponent;

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

    let update_time_config = {
        let system_config = system_config.clone();
        Callback::from(move |config| {
            system_config.dispatch(SystemConfigurationAction::SetTimeConfig(config));
        })
    };
    let update_aare_config = {
        let system_config = system_config.clone();
        Callback::from(move |config| {
            system_config.dispatch(SystemConfigurationAction::SetAareConfig(config));
        })
    };
    let update_cafete_config = {
        let system_config = system_config.clone();
        Callback::from(move |config| {
            system_config.dispatch(SystemConfigurationAction::SetCafeteConfig(config));
        })
    };
    let update_bernaqua_config = {
        let system_config = system_config.clone();
        Callback::from(move |config| {
            system_config.dispatch(SystemConfigurationAction::SetBernaquaConfig(config));
        })
    };
    let update_public_transport_config = {
        let system_config = system_config.clone();
        Callback::from(move |public_transport_config| {
            system_config.dispatch(SystemConfigurationAction::SetPublicTransportConfig(
                public_transport_config,
            ));
        })
    };
    let update_public_transport_base_config = {
        let system_config = system_config.clone();
        Callback::from(move |config| {
            system_config.dispatch(SystemConfigurationAction::SetPublicTransportBaseConfig(
                config,
            ));
        })
    };
    html! {
        <div>
            <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
            <div class="flex items-center justify-center h-screen w-screen bg-zinc-300">
                // Card
                <div class="bg-zinc-200 rounded-2xl p-5 shadow-2xl">
                    // Flex Container
                    <div class="flex flex-col">
                        // Image
                        <img src="assets/logo.png" alt="" class="h-24 object-contain"/>
                        // Content
                        <div class="p-3">
                            <p class="leading-5 text-5xl text-zinc-900 font-bold tracking-wide text-center py-10">
                                {"Configuration"}
                            </p>

                            <div>
                                <ConfigCardComponent>
                                    <DefaultWidgetConfigComponent
                                        meta_data={WidgetMetaData::Time}
                                        config={system_config.widget_config.time_config.clone()}
                                    on_change={update_time_config}
                                    />
                                </ConfigCardComponent>

                                <ConfigCardComponent>
                                    <DefaultWidgetConfigComponent
                                        meta_data={WidgetMetaData::Aare}
                                        config={system_config.widget_config.aare_config.clone()}
                                    on_change={update_aare_config}
                                    />
                                </ConfigCardComponent>

                                <ConfigCardComponent>
                                    <DefaultWidgetConfigComponent
                                        meta_data={WidgetMetaData::Cafete}
                                        config={system_config.widget_config.cafete_config.clone()}
                                    on_change={update_cafete_config}
                                    />
                                </ConfigCardComponent>

                                <ConfigCardComponent>
                                    <DefaultWidgetConfigComponent
                                        meta_data={WidgetMetaData::Bernaqua}
                                        config={system_config.widget_config.bernaqua_config.clone()}
                                    on_change={update_bernaqua_config}
                                    />
                                </ConfigCardComponent>

                                <ConfigCardComponent>
                                    <WidgetConfigComponent
                                        meta_data={WidgetMetaData::PublicTransport}
                                        config={system_config.widget_config.public_transport_config.base_config.clone()}
                                        on_change={update_public_transport_base_config}
                                    >
                                        <PublicTransportConfigComponent
                                            config={system_config.widget_config.public_transport_config.clone()}
                                            on_change={update_public_transport_config}
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

fn main() {
    yew::Renderer::<MainComponent>::new().render();
}
