use super::super::molecules::user::*;
use super::login;
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub set_logged_in: Callback<bool>,
}

#[function_component]
pub fn RegisterBox(props: &Props) -> Html {
    let user = use_state(|| User::default());
    let user_clone = user.clone();
    let set_logged_in = props.set_logged_in.clone();
    let submit = Callback::from(move |event: MouseEvent| {
        let user_clone = user_clone.clone();
        event.prevent_default();
        let set_logged_in = set_logged_in.clone();
        wasm_bindgen_futures::spawn_local(async move {
            register(&(*user_clone)).await;
            login::login(&(*user_clone)).await;
            set_logged_in.emit(login::check_logged_in());
        });
    });
    html! {
        <div class={classes!("LoginBox")}>
            {"Create Account"}
            <UserForm submit={submit} user={user}/>
        </div>
    }
}

async fn register(user: &User) {
    let url = std::env!("URL");
    let url = std::format!("http://{url}/register");
    let request = Request::post(&url).json(user);

    if let Ok(request) = request {
        let _ = request.send().await;
    }
}
