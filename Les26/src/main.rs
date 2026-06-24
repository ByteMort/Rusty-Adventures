mod app;
mod stores;
mod login_form;
mod display_auth;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
