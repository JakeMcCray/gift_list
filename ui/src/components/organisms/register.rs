use super::super::molecules::user::*;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component]
pub fn RegisterBox() -> Html {
    let user = use_state(|| User::default());
    let user_clone = user.clone();
    let submit = Callback::from(move |event: MouseEvent| {
        let user_clone = user_clone.clone();
        event.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            register(&(*user_clone)).await;
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
