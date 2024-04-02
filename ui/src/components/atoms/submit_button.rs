use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Submit(props: &Props) -> Html {
    html! {
        <button class={classes!("SubmitButton")} onclick={props.onclick.clone()}>{"Submit"}</button>
    }
}
