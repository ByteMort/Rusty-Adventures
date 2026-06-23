use stylist::{ Style, style };
use yew::prelude::*;
use gloo::console::log;

pub enum Msg {
    ButtonClicked(u32)
}

#[derive(Properties, PartialEq)]
pub struct Props{
    pub message: String,
}

pub struct StructHello{
    pub stylesheet: Style,
    pub count: u32,
}

impl StructHello{
    fn style() -> Style{
        style!(r#"
            color: cyan;
        "#)
        .unwrap()
    }
}

impl Component for StructHello{
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            stylesheet: Self::style(),
            count: 0
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <>
                <h1 class={self.stylesheet.clone()}>{&ctx.props().message}</h1>
                <div>
                    <button onclick={ctx.link().callback(
                        |_| Msg::ButtonClicked(2) 
                    )}>{"Click Me"}</button>
                    <p>{"I have been clicked "}{self.count}{" times."}</p>
                </div>
            </>
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClicked(amount) => {
                self.count += amount;
                true
            }
        }
    }
    
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if ctx.props().message != _old_props.message{
            log!("StrcutComponent Changed Rendered");
            true            
        }else{
            false
        }
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render{
            log!("StructComponent loaded with: ", ctx.props().message.clone());
        }
    }
    
    fn destroy(&mut self, _ctx: &Context<Self>) {
        log!("Destroyed");
    }
}