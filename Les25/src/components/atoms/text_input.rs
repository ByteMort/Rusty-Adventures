use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub name: String,
    pub placeholder: String,
    pub handle_onchange: Callback<String>
}

#[styled_component(TextInput)]
pub fn text_input(props: &Props) -> Html{
    let handle_onchange:Callback<String> = props.handle_onchange.clone();
    let onchange:Callback<Event> = Callback::from(move |e:Event|{
        let value:String = e.target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        handle_onchange.emit(value);
    });

    html!{
        <input type="text" placeholder={props.placeholder.clone()} name={props.name.clone()} 
            onchange={onchange} />
    }
}