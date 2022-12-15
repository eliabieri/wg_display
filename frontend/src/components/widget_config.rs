use yew::prelude::*;

use common::models::BaseWidgetConfig;

use crate::components::default_widget_config::DefaultWidgetConfigComponent;

use common::widget_meta_data::WidgetMetaData;

#[derive(Properties, PartialEq)]
pub struct WidgetConfigProps {
    pub meta_data: WidgetMetaData,
    pub config: BaseWidgetConfig,
    pub on_change: Callback<BaseWidgetConfig>,
    pub children: Children,
}

#[function_component(WidgetConfigComponent)]
pub fn widget_config_component(props: &WidgetConfigProps) -> Html {
    html! {
        <div class="flex flex-col gap-3">
            <DefaultWidgetConfigComponent
                meta_data={props.meta_data.clone()}
                config={props.config.clone()}
                on_change={props.on_change.clone()}
            />

            { for props.children.iter() }
        </div>
    }
}
