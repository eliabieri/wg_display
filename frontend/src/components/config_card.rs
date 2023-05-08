//! Card wrapper for a configuration block

use yew::prelude::*;

/// Props for the ConfigCardComponent
#[derive(Properties, PartialEq)]
pub struct ConfigCardProps {
    /// The children of this component. Pass a [WidgetConfigComponent](crate::components::widget_config::WidgetConfigComponent) here.
    pub children: Children,
}

#[function_component(ConfigCardComponent)]
pub fn config_card_component(props: &ConfigCardProps) -> Html {
    html! {
        <div class="p-4 my-3 transition duration-800 rounded-md bg-zinc-700 shadow-md">
            { for props.children.iter() }
        </div>
    }
}
