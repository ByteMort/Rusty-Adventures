mod components;
use components::atoms::main_title::{MainTitle, Color};

// use std::str::FromStr;
// use serde::{Deserialize, Serialize};
use stylist::{ /* Style, ast::Sheet, style,*/ yew::styled_component};
use yew::prelude::*;

use gloo::console::log;


// const STYLE_FILE:&str = include_str!("main.css");

/*
#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String
}
*/



#[styled_component(App)]
pub fn app() -> Html{

    let main_title_load = Callback::from(|message:String|{
        log!(message)
    });

    html!{
        <div>
            <MainTitle title="hi there" 
                color={Color::Error} 
                on_load={main_title_load}/>
        </div>
    }
}


/*

// #[function_component(App)]
#[styled_component(App)]
pub fn app() -> Html{
    let stylesheet = style!(
        r#"
            h1{
                color: orange;
            }
            p{
                color: purple;
            }
        "#
    ).unwrap();

    let stylessheet2 = Style::new(Sheet::from_str(STYLE_FILE).unwrap()).unwrap();

    let name:&str = "ByteMort";
    let my_object:MyObject = MyObject { username: name.to_string(), favorite_language: "Rust".to_owned() };

    log!("My name is", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let class:&str = "my_title";
    let message:Option<&str> = Some("I am a message");


    let tasks1:Vec<&str> = vec!["record video", "install patchs", "delete files"];
    let _tasks2:Vec<Html> = vec![html!{<li>{"record video"}</li>}, 
    html!{<li>{"install patchs"}</li>}, html!{<li>{"delete files"}</li>}];

    html! {
        <>
            <div class={classes!(stylesheet, stylessheet2)} >
                <h1>{"Hello World!!"}</h1>

                if class == "my_title"{
                    <p>{"Hi there!"}</p>
                }else{
                    <p>{"Nope!"}</p>
                }

                if let Some(message) = message {
                    <p class={css!("font-size: 25px;font-style:italic;")} >{message}</p>
                }else{
                    <p>{"There is no meesage."}</p>
                }

                <ul>
                    // {for tasks1.iter()}
                    // {_tasks2}
                    {list_to_html(tasks1)}
                </ul>
            </div>
        </>
    }
}

fn list_to_html(list:Vec<&str>) -> Vec<Html>{
    list.iter().map(|item| {
        html!{<li>{item}</li>}
    }).collect()
}

*/