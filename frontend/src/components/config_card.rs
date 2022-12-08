use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct WidgetConfigProps {
    id: String,
    children: Children,
}

#[function_component(DefaultWidgetConfigComponent)]
pub fn widget_config_component(props: &WidgetConfigProps) -> Html {
    html! {
        <div class="p-3 my-3 transition duration-800 outline outline-zinc-900 hover:outline-white hover:outline-2 rounded-sm">
            <div class="flex justify-between">
                <div id={props.id.clone()}>
                    { props.children.clone() }
                </div>
            </div>
        </div>
    }
}
