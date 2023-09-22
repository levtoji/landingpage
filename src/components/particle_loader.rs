use super::particle::*;
use rand::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub struct PositionStore {
    pub positions: Vec<Position>,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

fn get_random_position() -> Position {
    Position {
        x: (rand::thread_rng().gen_range(0..100) / 2 as u32) * 2,
        y: (rand::thread_rng().gen_range(0..98) / 2 as u32) * 2,
    }
}

#[function_component]
pub fn ParticleLoader() -> Html {
    let mut used_positions: Vec<Position> = vec![];
    let particles = (1..=1000)
        .map(|_| {
            let mut position = get_random_position();
            while used_positions.contains(&position) {
                position = get_random_position();
            }
            used_positions.push(position.clone());
            let props = ParticleProps {
                position_x: position.x,
                position_y: position.y,
                state: rand::thread_rng().gen_bool(0.5),
            };
            html!(<Particle ..props />)
        })
        .collect::<Html>();
    particles
}
