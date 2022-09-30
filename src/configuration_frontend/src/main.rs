use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use serde::{Deserialize, Serialize};

mod configuration;
use configuration::Configuration;

enum Msg {
    InitConfig(Configuration),
    SaveConfig(Configuration),
}

struct Model {
    config: Configuration,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            config: Configuration { example_value: String::from("") },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SaveConfig(config) => {
                self.config = config;
                let config_copy = self.config.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    Request::post("/config")
                        .json(&config_copy)
                        .expect("Could not serialize config")
                        .send()
                        .await
                        .unwrap();
                });
                true
            },
            Msg::InitConfig(config) => {
                self.config = config;
                true
            },
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("/config")
                    .send()
                    .await
                    .unwrap();

                let config = response
                    .json::<Configuration>()
                    .await
                    .unwrap();
                link.send_message(Msg::InitConfig(config));
            });
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_config_change = link.batch_callback(|e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            input.map(|input| Msg::SaveConfig(Configuration{example_value: input.value()}))
        });

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
                                    class="text-xs bg-lime-500 rounded-md p-2 text-zinc-800 hover:bg-lime-700 hover:text-white duration-500"
                                >
                                    {"Save"}
                                </button>

                                <p class="text-white mt-3 text-center">{"Current example config value: "}{ &self.config.example_value }</p>
                                </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
