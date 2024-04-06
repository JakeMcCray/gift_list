use crate::components::atoms::side_button::SideButton;
use crate::Page;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub buttons: Vec<Page>,
    pub state: UseStateHandle<Page>,
}

#[function_component]
pub fn Buttons(props: &Props) -> Html {
    let state = &props.state;
    let mut buttons: Vec<Html> = Vec::new();

    let height = 1.0 / (props.buttons.len() as f32);
    let height = ((height * 100.0) as u32).to_string();

    if !props.buttons.contains(&**state) {
        if let Some(p) = props.buttons.clone().first() {
            state.set(p.clone())
        }
    }

    for button in props.buttons.clone() {
        let state = state.clone();
        let pressed = button == **&state;
        let button_clone = button.clone();
        let onclick = Callback::from(move |_| {
            let button_clone = button_clone.clone();
            state.set(button_clone);
        });
        buttons.push(
            html! {<SideButton pressed={pressed} name={button.to_string()} onclick={onclick} height={height.clone()} />},
        );
    }

    html! {
        <>
            {for buttons}
        </>
    }
}
