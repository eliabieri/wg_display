//! Simple widget to display an error message.
use yew::{function_component, html, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct ErrorDisplayProps {
    pub error: UseStateHandle<Option<String>>,
}

#[function_component(ErrorDisplay)]
pub fn error_display_component(props: &ErrorDisplayProps) -> Html {
    let error = (*(props.error.clone())).clone();
    html! {
        if error.is_some() {
            <p class="text-red-800 text-center border border-black m-2 p-2 border-rou rounded-md">
                <span>{"Error: "}</span>
                <span>{error.unwrap()}</span>
            </p>
        }
    }
}
