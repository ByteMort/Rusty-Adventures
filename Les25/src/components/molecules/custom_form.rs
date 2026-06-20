use std::ops::Deref;

use yew::prelude::*;
use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::custom_button::CustomButton;

#[derive(Default, Clone)]
struct Data{
    pub username: String,
    pub count: u32,
}

#[function_component(CustomForm)]
pub fn custom_form() -> Html{
    let on_submit:Callback<SubmitEvent> = Callback::from(|e:SubmitEvent|{
        e.prevent_default();
    });

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

    html!{
        <>
            <form onsubmit={on_submit}>
                <TextInput name="username" handle_onchange={username_changed} />
                <CustomButton label="Count" onclick={button_clicked} />
                <p>{"Username: "}{&state.username}</p>
                <p>{"Button has been clicked "}{state.count} {" times."}</p>
            </form>
        </>
    }
}