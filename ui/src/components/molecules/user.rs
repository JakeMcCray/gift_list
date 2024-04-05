use super::super::atoms::submit_button::*;
use super::super::atoms::textbox::*;
use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize, Default, Clone, Debug, PartialEq)]
pub struct User {
    username: String,
    _email: Option<String>,
    password: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub submit: Callback<MouseEvent>,
    pub user: UseStateHandle<User>,
}

#[function_component]
pub fn UserForm(props: &Props) -> Html {
    let user_clone = props.user.clone();
    let change_username = Callback::from(move |name: String| {
        let mut user_copy = (*user_clone).clone();
        user_copy.username = name;
        user_clone.set(user_copy);
    });
    let user_clone = props.user.clone();
    let change_password = Callback::from(move |password: String| {
        let mut user_copy = (*user_clone).clone();
        user_copy.password = password;
        user_clone.set(user_copy);
    });

    html! {
            <form>
                <TextBox data={change_username} name={"Username"} /> <br />
                <TextBox data={change_password} name={"Password"} /> <br />
                <Submit onclick={props.submit.clone()}/>
            </form>
    }
}
