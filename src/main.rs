use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/:.._path")]
    Schedule { _path: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Settings {
    title: String,
    max_ahead: u64,
    lines: Vec<LineSettings>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LineSettings {
    id: String,
    stop_id: String,
    color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ApiStopStatus {
    events: Vec<ApiTripStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ApiTripStatus {
    scheduled_departure: i64,
    realtime_departure: 
}

async fn fetch_stop_status(
    stop_id: String,
    chateau_id: String,
) -> Result<ApiStopStatus, reqwest::Error> {
    let url = format!("https://birchdeparturesfromstop.catenarymaps.org/departures_at_stop?stop_id={stop_id}&chateau_id={chateau_id}&include_shapes=false");
    reqwest::get(&url).await?.json().await
}

#[component]
fn Schedule(_path: Vec<String>) -> Element {
    let encoded = use_route::<Route>().to_string();
    let trimmed = encoded.strip_prefix('/').unwrap_or(&encoded);
    let settings: Settings = ron::from_str(trimmed)?;
    rsx! {
        p { "{trimmed}" }
        p { "{settings:?}" }
    }
}
