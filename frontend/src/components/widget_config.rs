//! Component allowing for widgets to extend the default widget configuration.
//! Use this component if your widget has additional configuration options exceeding the default widget configuration.

use yew::prelude::*;

use common::models::BaseWidgetConfig;

use crate::components::default_widget_config::DefaultWidgetConfigComponent;

/// Props for the WidgetConfigComponent
#[derive(Properties, PartialEq)]
pub struct WidgetConfigProps {
    /// The widget meta data
    pub meta_data: WidgetMetaData,
    /// The base configuration of the widget
    pub config: BaseWidgetConfig,
    /// Callback to be called when the configuration changes
    pub on_change: Callback<BaseWidgetConfig>,
    /// The children of this component. Custom widget configuration components can be passed here.
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
