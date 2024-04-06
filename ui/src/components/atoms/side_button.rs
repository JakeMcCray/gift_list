use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub onclick: Callback<MouseEvent>,
    pub height: String,
    pub pressed: bool,
}

#[function_component]
pub fn SideButton(props: &Props) -> Html {
    if props.pressed {
        html! {
            <button onclick={props.onclick.clone()} style={format!("height: {}%; background: #FF6B6B" ,props.height.clone())}>{props.name.clone()}</button>
        }
    } else {
        html! {
            <button onclick={props.onclick.clone()} style={format!("height: {}%" ,props.height.clone())}>{props.name.clone()}</button>
        }
    }
}
