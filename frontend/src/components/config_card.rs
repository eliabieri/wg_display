use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ConfigCardProps {
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
