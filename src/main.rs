mod calculator;
mod button;

use calculator::{Calculator};

// use yew::prelude::*;

fn main() {
    yew::Renderer::<Calculator>::new().render();
}

