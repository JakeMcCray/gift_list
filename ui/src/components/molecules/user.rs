use super::super::atoms::submit_button::*;
use super::super::atoms::textbox::*;
use gloo_net::http::Request;
use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize, Default, Clone, Debug)]
pub struct User {
    username: String,
    _email: Option<String>,
    password: String,
}

#[function_component]
pub fn UserForm() -> Html {
    let user = use_state(|| User::default());
    let user_clone = user.clone();
    let submit = Callback::from(move |event: MouseEvent| {
        let user_clone = user_clone.clone();
        event.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            register(&(*user_clone)).await;
        });
    });
    let user_clone = user.clone();
    let change_username = Callback::from(move |name: String| {
        let mut user_copy = (*user_clone).clone();
        user_copy.username = name;
        user_clone.set(user_copy);
    });
    let user_clone = user.clone();
    let change_password = Callback::from(move |password: String| {
        let mut user_copy = (*user_clone).clone();
        user_copy.password = password;
        user_clone.set(user_copy);
    });

    html! {
            <form>
                <TextBox data={change_username} name={"Username"} /> <br />
                <TextBox data={change_password} name={"Password"} /> <br />
                <Submit onclick={submit}/>
            </form>
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
