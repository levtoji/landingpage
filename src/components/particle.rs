use gloo::timers::callback::Timeout;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ParticleProps {
    pub position_x: u32,
    pub position_y: u32,
    pub state: bool,
}

#[function_component]
pub fn Particle(props: &ParticleProps) -> Html {
    let one = use_state(|| props.state);
    let one_cloned = one.clone();
    use_effect(move || {
        Timeout::new(200, move || {
            one.set(!*one);
        })
        .forget();
    });
    html! {
        <>
        <div class="absolute -z-50 text-lime-500 hover:text-rose-700" style={format!{"left: {}%; top: {}%;", props.position_x, props.position_y}}>
            <p class="particle">{ if *one_cloned { "1" } else { "0" } }</p>
        </div>
        </>
    }
}
