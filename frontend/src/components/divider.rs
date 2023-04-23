//! Simple divider component

use yew::prelude::*;

/// Props for the DividerComponent
#[derive(Properties, PartialEq)]
pub struct DividerProps {
    /// The text to display in the divider
    pub text: AttrValue,
}

#[function_component(DividerComponent)]
pub fn divider_component(props: &DividerProps) -> Html {
    html! {
        <div class="relative mb-4">
            <div class="absolute inset-0 flex items-center" aria-hidden="true">
                <div class="w-full border-t border-gray-300"></div>
            </div>
            <div class="relative flex justify-center">
                <span class="bg-zinc-200 px-2 text-sm text-gray-500">{props.text.clone()}</span>
            </div>
    </div>
    }
}
