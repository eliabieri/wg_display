use yew::prelude::*;

use common::models::BaseWidgetConfig;

use crate::components::default_widget_config::DefaultWidgetConfigComponent;

#[derive(Properties, PartialEq)]
pub struct WidgetConfigProps {
    pub widget_name: AttrValue,
    pub config: BaseWidgetConfig,
    pub on_change: Callback<BaseWidgetConfig>,
    pub children: Children,
}

#[function_component(WidgetConfigComponent)]
pub fn widget_config_component(props: &WidgetConfigProps) -> Html {
    html! {
        <div class="flex flex-col gap-3">
            <DefaultWidgetConfigComponent
                widget_name={props.widget_name.clone()}
                config={props.config.clone()}
                on_change={props.on_change.clone()}
            />

            { for props.children.iter() }
        </div>
    }
}
