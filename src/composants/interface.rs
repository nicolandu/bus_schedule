use dioxus::prelude::*;
use chrono::{DateTime,Utc};

const STYLE_CSS: Asset = asset!("/src/composants/style.css");
const TIME_FORMAT: &str = "%H:%M";

#[component]
pub fn UserInterface()->Element {
    let time = use_signal(Utc::now);
    let id:Signal<String>=use_signal(||"0".to_string());//Exo, STM, RTM, STL
    // let chateau_id:Signal<String>=use_signal(||"0".to_string());//
    let color:Signal<Option<String>>=use_signal(||Some("red".to_string()));//Menu-déroulant
    let background_color:Signal<String>=use_signal(||"red".to_string());//Menu-déroulant
    let label:Signal<Option<String>>=use_signal(||Some("red".to_string()));//Menu-déroulant
    let priority:Signal<u32>=use_signal(||1);
    let scheduled:Signal<i64>=use_signal(||0);
    let path:String="".to_string();
    rsx!{
        document::Link { rel: "stylesheet", href: STYLE_CSS }
        body{   
            div { 
                class:"navbar",
                span{
                    class:"titre",
                    "Préférences de trajet"
                } span{
                    class:"heure",
                    {time.read().format(TIME_FORMAT).to_string()}
                }
            }
            div {
                class:"tableau",
                label {
                    for="setting",
                    
                }
                //id
                select {
                    name="service", id:"setting",
                    option{value="Exo", "Exo"}
                    option{value="STM", "STM"}
                    option{value="Exo", "Exo"}
                    option{value="Exo", "Exo"}
                    
                    "Nom du service"
                }
                //chateau_id onclick:move |_|{id} 
                // button {onclick:move |_|{}  }
                //color
                input{name="setting", }
                //background_color
                button {  }
                //label
                button {  }
                //priority
                button {  }
                //scheduled

            }
            div{
                class:"bouton_new_page",
                //Lien vers la page préférée
                Link{ to: "/{path}", span {  }}
            }}
    }
}