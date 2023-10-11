mod components;

use components::linking::*;
use components::particle_loader::*;
use components::pill::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
    <>
    <ParticleLoader />
    <div class="grid place-items-center h-screen">
    <div class="w-9/12 p-6 m-6 bg-white rounded-xl shadow-lg flex items-center bg-gradient-to-r from-orange-400 via-red-500 to-pink-500">
      <div class="space-y-4 drop-shadow-md">
        <div class="text-xl font-medium text-slate-100 text-center">{"Lev Perschin"}</div>
        <p class="text-slate-100">{"Welcome, I am a computer science student, currently working on my master thesis. For many years I worked in the university as a programmer, mostly using C# in the Unity Engine and Java for Android. Currently, I play around with Rust, which already is one of my favorites."}</p>
        <p class="text-slate-100">{"I worked for the institute for product management and technology at the technical university of Hamburg and have created teaching software in virtual and augmented reality. Through the years, we created tools for Microsoft HoloLens and the HTC VR Headsets and various tablets. The goal of the software was the creation of a teaching environment for workers in the industry as an alternative to traditional teaching methods."}</p>
        <p class="text-slate-100">{"I also worked on some private projects, which never were published as most time was spent on projects at work. My current favorite language is Rust, which i have created this site with, using yew and tailwindcss."}</p>
        <div class="flex items-baseline m-auto space-x-2 text-sm place-content-center">
            <Linking text="Xing" link="https://www.xing.com/profile/Lev_Perschin"><svg height="1.5em" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512">{"<!--! Font Awesome Free 6.4.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->"}<path fill="#FFFFFF" d="M162.7 210c-1.8 3.3-25.2 44.4-70.1 123.5-4.9 8.3-10.8 12.5-17.7 12.5H9.8c-7.7 0-12.1-7.5-8.5-14.4l69-121.3c.2 0 .2-.1 0-.3l-43.9-75.6c-4.3-7.8.3-14.1 8.5-14.1H100c7.3 0 13.3 4.1 18 12.2l44.7 77.5zM382.6 46.1l-144 253v.3L330.2 466c3.9 7.1.2 14.1-8.5 14.1h-65.2c-7.6 0-13.6-4-18-12.2l-92.4-168.5c3.3-5.8 51.5-90.8 144.8-255.2 4.6-8.1 10.4-12.2 17.5-12.2h65.7c8 0 12.3 6.7 8.5 14.1z"/></svg></Linking>
            <Linking text="Github" link="https://github.com/levtoji"><svg xmlns="http://www.w3.org/2000/svg" height="1.5em" viewBox="0 0 496 512">{"<!--! Font Awesome Free 6.4.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->"}<path fill="#FFFFFF" d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"/></svg> </Linking>
        </div>

        <div class="space-x-1 space-y-1 flex flex-wrap text-sm items-baseline m-auto">
            <Pill text="Rust" color="bg-lime-500" hover_color="hover:bg-lime-600"/>
            <Pill text="C# (Unity)" color="bg-lime-500" hover_color="hover:bg-lime-600"/>
            <Pill text="Python" color="bg-lime-500" hover_color="hover:bg-lime-600"/>
            <Pill text="Java (Android)" color="bg-lime-500" hover_color="hover:bg-lime-600"/>
            <Pill text="C/C++" color="bg-lime-500" hover_color="hover:bg-lime-600"/>
            <Pill text="PHP" color="bg-lime-500" hover_color="hover:bg-lime-600"/>
            <Pill text="HTML" color="bg-red-500" hover_color="hover:bg-red-600"/>
            <Pill text="CSS (Pure, Tailwind)" color="bg-red-500" hover_color="hover:bg-red-600"/>
            <Pill text="XML/YAML" color="bg-red-500" hover_color="hover:bg-red-600"/>
            <Pill text="Git" color="bg-red-500" hover_color="hover:bg-red-600"/>
            <Pill text="Augmented Reality" color="bg-red-500" hover_color="hover:bg-red-600"/>
            <Pill text="Virtual Reality" color="bg-red-500" hover_color="hover:bg-red-600"/>
            <Pill text="Blender" color="bg-yellow-500" hover_color="hover:bg-yellow-600"/>
            <Pill text="AutoCAD" color="bg-yellow-500" hover_color="hover:bg-yellow-600"/>
            <Pill text="FreeCAD" color="bg-yellow-500" hover_color="hover:bg-yellow-600"/>
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
