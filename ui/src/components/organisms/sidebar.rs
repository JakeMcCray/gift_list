use yew::prelude::*;

use super::super::atoms::side_button::*;

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
                <SideButton class={"ButtonDown"} name={"Login"} onclick={login} />
                <br />
                <SideButton class={""} name={"Register"} onclick={register} />
                }
            else{
                <SideButton class={""} name={"Login"} onclick={login} />
                <br />
                <SideButton class={"ButtonDown"} name={"Register"} onclick={register} />
            }
        </div>
    }
}
