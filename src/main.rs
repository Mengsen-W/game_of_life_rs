use yew::prelude::*;

mod color_utils;
mod components;
mod life;
mod settings;
use log::{error, info, warn};

use components::game::Game;
use settings::{default_settings, Settings};

#[function_component(App)]
fn app() -> Html {
    html! {
      <ContextProvider<Settings> context={default_settings()}>
        <Game/>
      </ContextProvider<Settings>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Starting the application...");
    yew::start_app::<App>();
}
