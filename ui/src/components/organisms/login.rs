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

//WARNING: I don't know how to do enviroment variable so i need to change this whenever my
//ip/domain changes
fn login() {
    todo!();
}
