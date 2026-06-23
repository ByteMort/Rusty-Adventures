use yew::prelude::*;

#[function_component(Hello)]
pub fn hello() -> Html{
    html!{
        <h1>{"This is Hello page."}</h1>
    }
}