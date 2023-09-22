mod components;

use components::particle_loader::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    html! {
        <>
        <ParticleLoader />
        <div class="grid place-items-center h-screen">
        <div class="w-9/12 p-6 bg-white rounded-xl shadow-lg flex items-center ">
          <div>
            <div class="text-xl font-medium text-black">{"Lev Perschin"}</div>
            <p class="text-slate-500">{"Welcome, I am a computer science student, currently working on my master thesis. For many years I worked in the university as a programmer, mostly using C# and Java. Currently, I play around with Rust, which already is one of my favorites."}</p>
          </div>
        </div>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
