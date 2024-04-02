use super::super::molecules::user::*;
use yew::prelude::*;

#[function_component]
pub fn RegisterBox() -> Html {
    html! {
        <div class={classes!("LoginBox")}>
            {"Create Account"}
            <UserForm/>
        </div>
    }
}
