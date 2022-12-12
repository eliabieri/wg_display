use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ConfigCardProps {
    pub children: Children,
}
#[function_component(ConfigCardComponent)]
pub fn config_card_component(props: &ConfigCardProps) -> Html {
    html! {
        <div class="p-3 my-3 transition duration-800 outline outline-zinc-900 hover:outline-white hover:outline-2 rounded-sm">
            { for props.children.iter() }
        </div>
    }
}
