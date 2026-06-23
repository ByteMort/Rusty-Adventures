use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::pages::home::Home;
use crate::components::pages::hello::Hello;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route{
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello
}

pub fn switch(route: &Route) -> Html{
    match route {
        Route::Home => html!{ <Home /> },
        Route::Hello => html!{ <Hello /> }
    }
}