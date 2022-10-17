use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use common::models::Configuration;

#[function_component(MainComponent)]
fn main_component() -> Html {
    let state = use_state(|| Configuration {
        example_value: String::from(""),
    });

    let state_copy = state.clone();
    let on_config_change = move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        let config = Configuration {
            example_value: input.unwrap().value(),
        };
        let config_copy = config.clone();
        state_copy.set(config);
        wasm_bindgen_futures::spawn_local(async move {
            Request::post("/config")
                .json(&config_copy)
                .expect("Could not serialize config")
                .send()
                .await
                .expect("Could not transmit config");
        });
    };

    {
        let state = state.clone();
        use_effect(|| {
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("/config")
                    .send()
                    .await
                    .expect("Config request failed");

                let config = response
                    .json::<Configuration>()
                    .await
                    .expect("Recevied config could not be deserialized");
                state.set(config);
            });
            || {}
        });
    }

    html! {
        <div class="flex items-center justify-center h-screen bg-zinc-700">
            // Card
            <div class="bg-zinc-800 rounded-2xl mx-6 p-2 shadow-2xl">
                // Flex Container
                <div class="flex flex-col md:flex-row">
                    // Image
                    <img src="assets/forest.jpg" alt="" class="rounded-xl h-80 md:h-64 hover:scale-110 duration-500 object-cover"/>
                    // Content
                    <div class="p-6 md:p-12">

                        <h2
                            class="text-white text-xl font-medium font-serif text-center md:text-left"
                        >
                            {"WG Display dashboard"}
                        </h2>

                        <p
                            class="text-white leading-5 my-4 text-xs tracking-wide text-center md:text-left"
                        >
                            {"Soon you'll be able to configure parameters here"}<br/>
                            {"Enter an example configuration value"}
                        </p>

                        <div
                            class="flex flex-col space-y-3 md:flex-row md:space-x-3 md:space-y-0"
                        >
                            <input
                                class="outline-2 border-zinc-600 border focus:border-none placeholder:text-xs content placeholder:text-center text-center text-zinc-500 bg-zinc-800"
                                placeholder="Enter the example value"
                                onchange={on_config_change}
                            />

                            <button
                                class="text-xs bg-lime-400 rounded-md p-2 text-zinc-800 hover:bg-lime-700 hover:text-white duration-500"
                            >
                                {"Save"}
                            </button>
                            </div>

                            <p class="text-white mt-3 text-center">{"Current example config value: "}{ &state.example_value }</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<MainComponent>();
}
