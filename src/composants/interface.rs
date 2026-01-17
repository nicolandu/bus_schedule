use dioxus::prelude::*;
use chrono::{DateTime,Utc};

const STYLE_CSS: Asset = asset!("/src/composants/style.css");
const TIME_FORMAT: &str = "%H:%M";

#[component]
pub fn UserInterface()->Element {
    let time = use_signal(Utc::now);
    
    rsx!{
        document::Link { rel: "stylesheet", href: STYLE_CSS }
        body{   
            div { 
                class:"navbar",
                // span{
                //     class:"titre",
                //     "Préférences de trajet"
                // } span{
                //     class:"heure",
                //     {time.read().format(TIME_FORMAT).to_string()}
                // }
            }
            div {
                class:"tableau",
                span {  }
                span {  }
                span {  }
                span {  }
                span {  }
                span {  }
            }
            div{
                class:"bouton_new_page",

            }}
    }
}