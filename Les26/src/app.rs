use crate::login_form::LoginForm;
use yew::prelude::*;
use crate::display_auth::DisplayAuth;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        

        html! {
            <div>
                <h1>{"App"}</h1>
                <LoginForm />

                <DisplayAuth />
            </div>
        }
    }
}
