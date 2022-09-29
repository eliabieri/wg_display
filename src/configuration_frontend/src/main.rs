use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
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
                                {"Sign up for the newsletter."}
                            </p>

                            <div
                                class="flex flex-col space-y-3 md:flex-row md:space-x-3 md:space-y-0"
                            >
                                <input
                                    class="outline-2 border-zinc-600 border focus:border-none placeholder:text-xs content placeholder:text-center text-center text-zinc-500 bg-zinc-800"
                                    placeholder="Enter your email address"
                                    type="email"
                                />

                                <button
                                    onclick={link.callback(|_| Msg::AddOne)}
                                    class="text-xs bg-lime-500 rounded-md p-2 text-zinc-800 hover:bg-lime-700 hover:text-white duration-500"
                                >
                                    {"Subscribe"}
                                </button>

                                <p class="text-white mt-3 text-center">{ self.value }</p>
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
