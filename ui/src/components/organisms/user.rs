use yew::prelude::*;

#[function_component]
pub fn User() -> Html {
    html! {
        <div class={classes!("LoginBox")}>
            {"This is where the users own wishlist will be managed"}
        </div>
    }
}
