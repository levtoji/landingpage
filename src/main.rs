mod components;

use components::linking::*;
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
      <div class="space-y-4">
        <div class="text-xl font-medium text-black">{"Lev Perschin"}</div>
        <p class="text-slate-600">{"Welcome, I am a computer science student, currently working on my master thesis. For many years I worked in the university as a programmer, mostly using C# and Java. Currently, I play around with Rust, which already is one of my favorites."}</p>
        <p class="text-slate-600">{"I worked for the institute for product management and technology at the technical university of Hamburg and have created teaching software in virtual and augmented reality. Through the years, we created tools for Microsoft HoloLens and the HTC VR Headsets. The aim of the software was the creation of a teaching environment for workers in the industry as an alternative to traditional teaching methods."}</p>
        <p class="text-slate-600">{"I also worked on some private projects, which never were published as most time was spent on projects at work. My current favorite language is Rust, which i have created this site with, also using tailwindcss."}</p>
        <div class="flex items-baseline mt-4">
        <div class="space-x-2 flex text-sm">
            <Linking text="Xing" link="https://www.xing.com/profile/Lev_Perschin"/>
            <Linking text="Github" link="https://github.com/levtoji"/>
        </div>
        </div>
      </div>
    </div>
    </div>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
