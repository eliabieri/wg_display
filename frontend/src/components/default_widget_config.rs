use gloo_console::log;
use yew::prelude::*;

use common::models::DefaultWidgetConfig;

#[derive(Properties, PartialEq)]
pub struct WidgetConfigProps {
    pub widget_name: AttrValue,
    pub config: DefaultWidgetConfig,
    pub on_change: Callback<DefaultWidgetConfig>,
}

#[function_component(DefaultWidgetConfigComponent)]
pub fn default_widget_config_component(props: &WidgetConfigProps) -> Html {
    let on_change = props.on_change.clone();
    let name_clone = props.widget_name.clone();
    let enabled = props.config.enabled;
    let toggle_enabled = move |_: MouseEvent| {
        match enabled {
            true => log!(format!("Disabling {}", name_clone)),
            false => log!(format!("Enabling {}", name_clone)),
        }
        on_change.emit(DefaultWidgetConfig { enabled: !enabled });
    };

    html! {
        <div class="p-3 my-3 transition duration-800 outline outline-zinc-900 hover:outline-white hover:outline-2 rounded-sm">
            <div class="flex justify-between">
                <div class="text-white text-md font-medium">{{ props.widget_name.as_str()}}</div>
                <input type={"checkbox"} checked={props.config.enabled} onclick={toggle_enabled}
                />
            </div>
        </div>
    }
}
