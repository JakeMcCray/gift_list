use yew::prelude::*;

#[derive(Default, Clone)]
pub struct User {
    username: String,
    email: Option<String>,
    password: String,
}

#[function_component]
pub fn UserForm() -> Html {
    let user = use_state(|| User::default());
    let user_clone = user.clone();
    let on_register = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
    });

    //TODO: undo these unwraps later
    let cloned_user = user.clone();
    let change_username = Callback::from(move |event: Event| {
        let username = event
            .target()
            .unwrap()
            .as_string()
            .unwrap_or("".to_string());
        let mut user_data = (*cloned_user).clone();
        user_data.username = username;
        cloned_user.set(user_data);
    });
    html! {
            <form>
                <input class={classes!("TextBox")} placeholder={"Username"} /> <br />
                <input class={classes!("TextBox")} placeholder={"Email"} /> <br />
                <input class={classes!("TextBox")} placeholder={"Password"} /> <br />
                <input type="submit" class={classes!("TextBox")} value="Submit" />
            </form>
    }
}
