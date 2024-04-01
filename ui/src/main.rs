use yew::prelude::*;

pub mod components;

use components::organisms::login::*;
use components::organisms::register::*;
use components::organisms::sidebar::*;

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| ButtonState::Login);
    html! {
        <>
            <Sidebar button_state={state.clone()}/>
            if *state == ButtonState::Login{
                <LoginBox />
            }
            else{
                <RegisterBox />
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
