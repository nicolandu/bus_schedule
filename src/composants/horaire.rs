//Hotreloader la page ou l'api every 30 seconds. Repreprendre le lien de l'api.

use dioxus::prelude::*;
use std::time::Duration;
use chrono::{DateTime, Local};

pub type Direction=String;
pub type LocalTime=DateTime<Local>;
#[derive(Clone, PartialEq)]
pub struct Trajet {
    // pub nom_déplacement: String, //Le nom de l'autobus
    pub numero_autobus: u8, //Le numéro de l'autobus
    pub direction: Direction, //Où est ce que l'autobus se dirige
    pub heure_arrivee: DateTime<Local>,
    // pub temps_avant_arret: SystemTime,
    //Autre temps
}


impl Trajet {
    fn get(&self)-> Self{
        self.clone()
    }
    pub fn get_numero(&self) -> u8{
        self.numero_autobus
    }
    pub fn get_direction(&self) -> Direction{
        self.direction.clone()
    }
    pub fn get_time_left(&self) -> String{
        let heure_arrivee:LocalTime=self.heure_arrivee;
        let temps_restant=Local::now()-heure_arrivee;
        format!("{:?},{:?}", heure_arrivee, temps_restant)

    }

    fn set_time(mut self, new_time:LocalTime)-> Self{
        self.heure_arrivee=new_time; self
    }
}
//Ticker de 30 secondes
async fn ticker_30( app_time:&mut LocalTime)->Signal<()>{
    use std::thread::sleep;
    let time=Local::now();
    sleep(Duration::new(30,0));
    *app_time= time;
    use_signal(move ||  dioxus::core::needs_update())
}

use dioxus::prelude::*;
use crate::reload_eevery_30_seconds::*;
use chrono::{DateTime, Local};
#[component]
pub fn Horaire(trajets:Vec<Trajet>)->Element {
    rsx!{
        //Horaire
        div {  
            for train in trajets{
                //Éléments affichés dans le trajet
                // div {train.nom_déplacement }
                span {"{train.get_numero()}" }
                span {"{train.get_direction()}"}
                span {"{train.get_time_left()}"}
                // span {"{temps_restant:?}"}
                
            }
        }
    }
}