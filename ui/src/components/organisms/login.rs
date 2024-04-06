use gloo::console::log;
use gloo_net::http::Request;
use yew::classes;
use yew::prelude::*;

use super::super::molecules::user::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub set_logged_in: Callback<bool>,
}

#[function_component]
pub fn LoginBox(props: &Props) -> Html {
    let user = use_state(|| User::default());
    let user_clone = user.clone();

    let set_logged_in = props.set_logged_in.clone();
    let submit = Callback::from(move |event: MouseEvent| {
        let user_clone = user_clone.clone();
        event.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            login(&(*user_clone)).await;
        });

        set_logged_in.emit(check_logged_in());
    });
    html! {
        <div class={classes!("LoginBox")}>
            {"Login"}
            <UserForm submit={submit} user={user}/>
        </div>
    }
}

async fn login(user: &User) {
    let url = std::env!("URL");
    let url = std::format!("http://{url}/login");
    let request = Request::post(&url).json(user);

    if let Ok(request) = request {
        let _ = request.send().await;
    } else if let Err(e) = request {
        let e: &str = &e.to_string();
        log!(e);
    }
}

fn check_logged_in() -> bool {
    log!("checking for cookies");
    match wasm_cookies::get_raw("user").is_some() {
        true => {
            log!("found user cookie");
            true
        }
        false => {
            log!("didn't find user cookie");
            false
        }
    }
}
