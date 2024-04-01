use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub button_state: UseStateHandle<ButtonState>,
}

#[derive(PartialEq)]
pub enum ButtonState {
    Login,
    Register,
}

#[function_component]
pub fn Sidebar(props: &Props) -> Html {
    let state = &props.button_state;
    let login = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(ButtonState::Login);
        })
    };

    let register = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(ButtonState::Register);
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
