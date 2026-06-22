use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub label: String,
    pub onclick: Callback<()>,
}

#[function_component(CustomButton)]
pub fn custom_form(props: &Props) -> Html{
    /*let copy_onclick:Callback<()> = props.onclick.clone();
    let button_onclick:Callback<MouseEvent> = Callback::from(move |_| {
        copy_onclick.emit(());
    });*/

    html!{
        <button /*onclick={button_onclick}*/>{&props.label}</button>
    }
}