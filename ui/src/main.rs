use gloo::console::log;
use strum::Display;
use yew::prelude::*;

pub mod components;

use components::organisms::groups::*;
use components::organisms::list::*;
use components::organisms::login::*;
use components::organisms::register::*;
use components::organisms::sidebar::*;
use components::organisms::user::*;

#[derive(Display, PartialEq, Clone)]
pub enum Page {
    Login,
    Register,
    User,
    Groups,
    List,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Page::Login);
    let logged_in = use_state(|| check_logged_in());

    let set_logged_in = {
        let logged_in = logged_in.clone();
        Callback::from(move |a| {
            log!(format!("setting logged_in status to {}", a));
            logged_in.set(a);
        })
    };

    html! {
        <>
            <Sidebar logged_in={logged_in.clone()} button_state={state.clone()}/>
            <Content page={state.clone()} set_logged_in={set_logged_in} />
        </>
    }
}

#[derive(Properties, PartialEq)]
struct Props {
    page: UseStateHandle<Page>,
    set_logged_in: Callback<bool>,
}

#[function_component]
fn Content(props: &Props) -> Html {
    let page = (*props.page).clone();
    match page {
        Page::Login => html! {<LoginBox set_logged_in={props.set_logged_in.clone()} />},
        Page::Register => html! {<RegisterBox set_logged_in={props.set_logged_in.clone()} />},
        Page::User => html! {<User />},
        Page::Groups => html! {<Groups />},
        Page::List => html! {<List />},
    }
}
fn check_logged_in() -> bool {
    wasm_cookies::get_raw("user").is_some()
}
fn main() {
    yew::Renderer::<App>::new().render();
}
