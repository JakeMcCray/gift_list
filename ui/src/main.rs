use yew::prelude::*;

pub mod login;
use crate::login::*;

pub mod sidebar;
use crate::sidebar::*;

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| sidebar::ButtonState::Login);
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
