use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub onclick: Callback<MouseEvent>,
    pub class: String,
}

#[function_component]
pub fn SideButton(props: &Props) -> Html {
    html! {
        <button onclick={props.onclick.clone()} class={props.class.clone()}>{props.name.clone()}</button>
    }
}
