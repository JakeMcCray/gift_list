use gloo::console::log;
use yew::prelude::*;

use super::super::molecules::buttons::*;
use crate::Page;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub logged_in: UseStateHandle<bool>,
    pub button_state: UseStateHandle<Page>,
}

#[function_component]
pub fn Sidebar(props: &Props) -> Html {
    let state = &props.button_state;

    let buttons = use_state(|| generate_buttons(*props.logged_in));

    let buttons = buttons.clone();
    html! {
        <div class={classes!("Sidebar")}>
            <Buttons buttons={(*buttons).clone()} state={state.clone()} />
        </div>
    }
}

fn generate_buttons(logged_in: bool) -> Vec<Page> {
    match logged_in {
        true => {
            log!("User is logged IN");
            vec![Page::User, Page::Groups, Page::List]
        }
        false => {
            log!("User is logged OUT");
            vec![Page::Login, Page::Register]
        }
    }
}
