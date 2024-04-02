use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub data: Callback<String>,
}

#[function_component]
pub fn TextBox(props: &Props) -> Html {
    let data = props.data.clone();

    let onchange = Callback::from(move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            data.emit(input.value());
        }
    });

    html! {
        <input onchange={onchange} class={classes!("TextBox")} placeholder={props.name.clone()} />
    }
}
