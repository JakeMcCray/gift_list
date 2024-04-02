use yew::classes;
use yew::prelude::*;

use super::super::molecules::user::*;

#[function_component]
pub fn LoginBox() -> Html {
    html! {
        <div class={classes!("LoginBox")}>
          {"Login"}
            <UserForm/>
        </div>
    }
}
