use yew::prelude::*;

use common::models::DefaultWidgetConfig;

#[derive(Properties, PartialEq)]
pub struct WidgetConfigProps {
    pub widget_name: String,
    pub config: DefaultWidgetConfig,
    // pub on_config_change: fn(DefaultWidgetConfig),
}

#[function_component(DefaultWidgetConfigComponent)]
pub fn default_widget_config_component(props: &WidgetConfigProps) -> Html {
    let enabled = use_state(|| props.config.enabled);

    // let toggle_enabled = |_: MouseEvent| {
    //     let enabled = enabled.clone();
    //     Callback::from(move |_: MouseEvent| enabled.set(!*enabled));
    //     // (props.on_config_change)(DefaultWidgetConfig { enabled: *enabled });
    // };

    html! {
        <div class="p-3 my-3 transition duration-800 outline outline-zinc-900 hover:outline-white hover:outline-2 rounded-sm">
            <div class="flex justify-between">
                <div class="text-white text-md font-medium">{{ props.widget_name.as_str() }}</div>
                <input type={"checkbox"} checked={props.config.enabled} //onclick={toggle_enabled}
                />
            </div>
        </div>
    }
}
