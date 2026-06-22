use std::ops::Deref;

use yew::prelude::*;
use crate::User;
use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::custom_button::CustomButton;

#[derive(Default, Clone)]
pub struct Data{
    pub username: String,
    pub count: u32,
    pub favorite_language: String,
}

#[derive(Properties, PartialEq)]
pub struct Props{
    pub onsubmit: Callback<Data>
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html{
    let user_context = use_context::<User>();

    // let username_state:UseStateHandle<String> = use_state(|| "No username set".to_string());
    // let button_count_state:UseStateHandle<u32> = use_state(|| 0);

    // let cloned_username_state:UseStateHandle<String> = username_state.clone();
    // let cloned_button_count_state:UseStateHandle<u32> = button_count_state.clone();

    // let username_changed:Callback<String> = Callback::from(move |username| {
        // cloned_username_state.set(username);
    // });

    // let button_clicked:Callback<()> = Callback::from(move |_| {
        // cloned_button_count_state.set(*cloned_button_count_state + 1);        
    // });
    
    // Solving state clone issue with one state
    let state:UseStateHandle<Data> = use_state(|| Data::default());

    let cloned_state:UseStateHandle<Data> = state.clone();
    let username_changed:Callback<String> = Callback::from(move |username|{
        // let mut data:Data = cloned_state.deref().clone();
        // data.username = username;
        // cloned_state.set(data);
        cloned_state.set(
            Data{ 
                username,
                ..cloned_state.deref().clone()
            }
        );
    });

    let cloned_state:UseStateHandle<Data> = state.clone();
    let button_clicked:Callback<()> = Callback::from(move |_|{
        cloned_state.set(
            Data{ 
                count: cloned_state.deref().clone().count + 1, 
                ..cloned_state.deref().clone()
            }
        );
    });


    let cloned_state:UseStateHandle<Data> = state.clone();
    let language_changed: Callback<String> = Callback::from(move |language: String|{
        cloned_state.set(
            Data{
                favorite_language: language,
                ..cloned_state.deref().clone()
            }
        );
    });


    let submit_clone:Callback<Data> = props.onsubmit.clone();
    let cloned_state:UseStateHandle<Data> = state.clone();
    let on_submit:Callback<SubmitEvent> = Callback::from(move |e: SubmitEvent|{
        e.prevent_default();
        let data = cloned_state.deref().clone();
        submit_clone.emit(data);
    });

    html!{
        <>
            <form onsubmit={on_submit}>
                <TextInput name="username" placeholder="Username" handle_onchange={username_changed} />
                <TextInput name="language" placeholder="Language" handle_onchange={language_changed} />
                <CustomButton label="Submit" onclick={button_clicked} />
                <p>{"Username: "}{&state.username}</p>
                // <p>{"Button has been clicked "}{state.count} {" times."}</p>
                if let Some(user) = user_context{
                    <p>{"User: "}{user.username}{", "}{user.fav_language}</p>
                }
            </form>
        </>
    }
}