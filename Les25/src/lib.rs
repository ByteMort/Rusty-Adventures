#![allow(non_snake_case)]
mod components;
use std::ops::Deref;

use components::atoms::main_title::{MainTitle, Color};
use components::molecules::custom_form::CustomForm;
// use std::str::FromStr;
// use serde::{Deserialize, Serialize};
use stylist::{ /* Style, ast::Sheet, style,*/ yew::styled_component};
use yew::prelude::*;
use yew::ContextProvider;
use gloo::console::log;
use crate::components::molecules::custom_form::Data;
mod router;
use crate::router::{switch, Route};
use yew_router::prelude::*;


// const STYLE_FILE:&str = include_str!("main.css");

/*
#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String
}
*/

#[derive(Clone, PartialEq, Default)]
pub struct User{
    pub username: String,
    pub fav_language: String,
}

#[styled_component(App)]
pub fn app() -> Html{

    let main_title_load = Callback::from(|message:String|{
        log!(message)
    });

    /*
    let customform_submit:Callback<Data> = Callback::from(|data: Data| {
        log!("Username: ", data.username, " Language: ", data.favorite_language);
    });
    */

    let first_load: UseStateHandle<bool> = use_state(|| true);
    use_effect(move || {
        // the code will run in first render and in all re-renders

        if *first_load{
            // Do ur task in first load
            first_load.set(false);
        }
    });

    let user_state:UseStateHandle<User> = use_state(|| User::default());
    let customform_submit:Callback<Data> = {
        let user_state:UseStateHandle<User> = user_state.clone();

        Callback::from(move |data: Data| {
            // log!("Username: ", data.username, " Language: ", data.favorite_language);
            let mut user:User = user_state.deref().clone();
            user.username = data.username;
            user.fav_language = data.favorite_language;
            user_state.set(user);
        })
    };
    // let user:User = User{username: "Mortwain".to_owned(), fav_language: "Rust".to_owned()};

    html!{
        <ContextProvider<User> /*context={user}*/ context={user_state.deref().clone()}>
            <MainTitle title="hi there" 
                color={Color::Ok} 
                on_load={main_title_load}/>
            <CustomForm onsubmit={customform_submit} />
            <BrowserRouter>
                <Switch <Route> render={|route: Route| switch(&route)} />
            </BrowserRouter>
            
        </ContextProvider<User>>
    }
}



// #[function_component(App)]
//#[styled_component(App)]
//pub fn app() -> Html{
    //let stylesheet = style!(
        //r#"
            //h1{
                //color: orange;
            //}
            //p{
                //color: purple;
            //}
        //"#
    //).unwrap();

    //let stylessheet2 = Style::new(Sheet::from_str(STYLE_FILE).unwrap()).unwrap();

    //let name:&str = "ByteMort";
    //let my_object:MyObject = MyObject { username: name.to_string(), favorite_language: "Rust".to_owned() };

    //log!("My name is", name);
    //log!(serde_json::to_string_pretty(&my_object).unwrap());

    //let class:&str = "my_title";
    //let message:Option<&str> = Some("I am a message");


    //let tasks1:Vec<&str> = vec!["record video", "install patchs", "delete files"];
    //let _tasks2:Vec<Html> = vec![html!{<li>{"record video"}</li>}, 
    //html!{<li>{"install patchs"}</li>}, html!{<li>{"delete files"}</li>}];

    //html! {
        //<>
            //<div class={classes!(stylesheet, stylessheet2)} >
                //<h1>{"Hello World!!"}</h1>

                //if class == "my_title"{
                    //<p>{"Hi there!"}</p>
                //}else{
                    //<p>{"Nope!"}</p>
                //}

                //if let Some(message) = message {
                    //<p class={css!("font-size: 25px;font-style:italic;")} >{message}</p>
                //}else{
                    //<p>{"There is no meesage."}</p>
                //}

                //<ul>
                    //// {for tasks1.iter()}
                    //// {_tasks2}
                    //{list_to_html(tasks1)}
                //</ul>
            //</div>
        //</>
    //}
//}

//fn list_to_html(list:Vec<&str>) -> Vec<Html>{
    //list.iter().map(|item| {
        //html!{<li>{item}</li>}
    //}).collect()
//}

