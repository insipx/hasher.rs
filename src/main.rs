use yew::prelude::*;
use yew::html::Scope;
use primitives::twox_128;
use yew::web_sys::HtmlInputElement as InputElement;
use wasm_bindgen::JsCast;

mod styles;

enum Msg {
    Add(String),
    Compute
}

struct Model {
    // `ComponentLink` is like a reference to a component.    
    // It can be used to send messages to the component
    link: Scope<Self>,
    parts: Vec<String>,
    key: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { link: ctx.link().clone(), parts: Vec::new(), key: String::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Compute => { 
                let parts: Vec<String> = self.parts.drain(..).collect();
                let key = generate_key(parts.iter().map(|s| s.as_str()).collect::<Vec<&str>>().as_slice());
                log::info!("Key: {}", key);
                self.key = key;
                true
            },
            Msg::Add(s) => {
                println!("Pushing!");
                self.parts.push(s);
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1 class = { "d-flex justify-content-center" }> { "Substrate Twox_128 Storage Key Hasher" } </h1>
                <div class= { "container-fluid" } >
                    <div>{ self.view_input(ctx.link()) }</div>
                </div>
                <div class = "container-fluid" >
                    <button onclick={ self.link.callback(|_| Msg::Compute) }>{ "Compute Full Key" }</button>
                    <h5>{ format!("Key for prefix: {}", &self.parts.join("")) }</h5>
                    <p>{ &self.key }</p>
                </div>
            </>
        }
    }
}

impl Model {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                log::info!("Input {:?}", value);
                input.set_value("");
                Some(Msg::Add(value))
            } else {
                None
            }
        });
        html! {
            <input
                class="form-control form-control-sm"
                placeholder= { format!("Place key here then press `Enter`") }
                {onkeypress}
            />
        }
    } 
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default()); 
    yew::start_app::<Model>();
}

pub fn generate_key(keys: &[&str]) -> String {
    keys.iter().map(|s| hex::encode(twox_128(s.as_bytes()))).collect()
}
