use yew::prelude::*;

use common::models::BaseWidgetConfig;

#[derive(Properties, PartialEq)]
pub struct DefaultWidgetConfigProps {
    pub widget_name: AttrValue,
    pub config: BaseWidgetConfig,
    pub on_change: Callback<BaseWidgetConfig>,
}

#[function_component(DefaultWidgetConfigComponent)]
pub fn default_widget_config_component(props: &DefaultWidgetConfigProps) -> Html {
    let on_change = props.on_change.clone();
    let enabled = props.config.enabled;
    let toggle_enabled = move |_: MouseEvent| {
        on_change.emit(BaseWidgetConfig { enabled: !enabled });
    };

    html! {
        <div class="flex justify-between">
            <div class="text-white text-md font-medium">{{ props.widget_name.as_str()}}</div>
            <input type={"checkbox"} checked={props.config.enabled} onclick={toggle_enabled}
            />
        </div>
    }
}
