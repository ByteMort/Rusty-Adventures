use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::atoms::struct_hello::StructHello;

#[function_component(Hello)]
pub fn hello() -> Html{
    let navigator:Navigator = use_navigator().unwrap();
    let onclick:Callback<_> = Callback::from(move |_| {
        navigator.clone().push(&Route::Home);
    });
    
    let text_state:UseStateHandle<String> = use_state(|| "Hello from lib.rs".to_owned());
    let onclick2:Callback<_> = {
        let text_state:UseStateHandle<String> = text_state.clone();
        Callback::from(move |_| {
            text_state.set("Message has been changed!".to_owned());
        })
    };

    html!{
        <div>
            <h1>{"This is Hello page."}</h1>
            <Link<Route> to={Route::Home}>{"Go to Home Page."}</Link<Route>>
            <button onclick={onclick}>{"Go Home"}</button>
            <button onclick={onclick2}>{"Change Message"}</button>
            <StructHello message={(*text_state).clone()} />
        </div>
    }
}