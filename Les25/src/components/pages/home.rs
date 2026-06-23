use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Home)]
pub fn home() -> Html{
    html!{
        <div>
            <h1>{"This is Home page."}</h1>
            <Link<Route> to={Route::Hello}>{"Go to Hello Page."}</Link<Route>>
        </div>
    }
}