use yew::classes;
use yew::prelude::*;

#[function_component]
pub fn LoginBox() -> Html {
    html! {
        <div class={classes!("LoginBox")}>
          {"Login"}
              <form>
                <input class={classes!("TextBox")} placeholder={"Username"} /> <br />
                <input class={classes!("TextBox")} placeholder={"Password"} /> <br />
                <input type="submit" class={classes!("TextBox")} value="Submit" />
            </form>
        </div>
    }
}

#[function_component]
pub fn RegisterBox() -> Html {
    html! {
        <div class={classes!("RegisterBox")}>
          {"Register"}
              <form>
                <input class={classes!("TextBox")} placeholder={"Username"} /> <br />
                <input class={classes!("TextBox")} placeholder={"Email"} /> <br />
                <input class={classes!("TextBox")} placeholder={"Password"} /> <br />
                <input type="submit" class={classes!("TextBox")} value="Submit" />
            </form>
        </div>
    }
}
