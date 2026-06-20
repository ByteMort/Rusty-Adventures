use stylist::{Style, style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Probs{
    // #[prop_or_default]
    #[prop_or("Hellllo".to_string())]
    pub title: String,

    pub color: Color,

    pub on_load: Callback<String>,
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Color{
    Normal,
    Ok,
    Error,
}
impl Color {
    pub fn to_string_from_class(&self) -> String{
        match self {
            Color::Normal => "normal".to_string(),
            Color::Ok => "ok".to_string(),
            Color::Error => "error".to_string(),
        }
    }
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Probs) -> Html{
    let stylesheet:Style = style!(
        r#"
            .normal{
                color: white;
            }
            .ok{
                color: green;
            }
            .error{
                color: red;
            }
        "#
    ).unwrap();

    props.on_load.emit("I loaded.".to_owned());

    html!{
        <div class={stylesheet}>
            <h1 class={&props.color.to_string_from_class()}>{&props.title}</h1>
        </div>
    }
}