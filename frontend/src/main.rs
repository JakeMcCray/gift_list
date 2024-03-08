use gloo::console::log;
use yew::prelude::*;

pub mod login;
use crate::login::*;

#[derive(PartialEq)]
enum ButtonState {
    Login,
    Register,
}

#[derive(Properties, PartialEq)]
struct Props {
    button_state: UseStateHandle<ButtonState>,
}

#[function_component(Sidebar)]
fn sidebar(props: &Props) -> Html {
    let state = &props.button_state;
    let login = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(ButtonState::Login);
            log!("state changed");
        })
    };

    let register = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(ButtonState::Register);
            log!("state changed");
        })
    };

    html! {
        <div class={classes!("Sidebar")}>
            if **state == ButtonState::Login{
                <button class={"ButtonDown"}>
                    {"Login"}
                </button>
                <br />
                <button onclick={register} class={""}>
                    {"Register"}
                </button>
                }
            else{
                <button onclick={login} class={""}>
                    {"Login"}
                </button>
                <br />
                <button class={"ButtonDown"}>
                    {"Register"}
                </button>
            }
        </div>
    }
}

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
