// http://127.0.0.1:8080/?(title:%22Prochains%20bus%22,max_ahead:14400,lines:[(id:%2251%22,label:Some(%22Est%22),stop_id:%2251167%22,chateau_id:%22soci%C3%A9t%C3%A9~de~transport~de~montr%C3%A9al%22,color:Some(%22%23ffffff%22),background_color:Some(%22%2322bbff%22),priority:0)])
use std::time::Duration;

use chrono::{DateTime, Local, TimeDelta, Utc};
use futures::future::join_all;

use dioxus::{core::anyhow, document::Link, prelude::*};
use serde::{Deserialize, Serialize};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const LIVE_SVG: Asset = asset!("/assets/live.svg");

const RELOAD_DURATION: Duration = Duration::from_secs(30);
const UPDATE_DELTA: Duration = Duration::from_secs(1);
const MAX_TRIPS_SHOWN: usize = 3;
const TIME_FORMAT: &str = "%H:%M";

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/?:..params")]
    Schedule { params: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum TripStatus {
    Cancelled,
    NoRealtime,
    Realtime(i64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Settings {
    title: String,
    max_ahead: u32,
    lines: Vec<LineSettings>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct LineSettings {
    id: String,
    stop_id: String,
    chateau_id: String,
    color: Option<String>,
    background_color: Option<String>,
    outline_color: Option<String>,
    label: Option<String>,
    priority: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StopApiStatus {
    events: Vec<ApiTrip>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ApiTrip {
    route_id: String,
    scheduled_departure: i64,
    realtime_departure: Option<i64>,
    stop_cancelled: bool,
    trip_cancelled: bool,
    trip_deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct LineStatus {
    settings: LineSettings,
    trips: Vec<Trip>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct Trip {
    scheduled: i64,
    status: TripStatus,
}

impl Ord for Trip {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let time_a = if let TripStatus::Realtime(t) = self.status {
            t
        } else {
            self.scheduled
        };
        let time_b = if let TripStatus::Realtime(t) = other.status {
            t
        } else {
            other.scheduled
        };
        time_a.cmp(&time_b)
    }
}

impl PartialOrd for Trip {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

async fn fetch_stops(
    lines: &[LineSettings],
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<LineStatus>, reqwest::Error> {
    info!("start fetch");
    let futures = lines.iter().map(|line| {
        let line_clone = line.clone();
        async move {
            let api_data =
                fetch_stop_status(&line_clone.stop_id, &line_clone.chateau_id, start, end)
                    .await
                    .expect("Fetch error");

            let mut trips = api_data
                .events
                .iter()
                .filter(|event| event.route_id == line_clone.id)
                .map(|event| {
                    let cancelled =
                        event.stop_cancelled || event.trip_cancelled || event.trip_deleted;
                    Trip {
                        scheduled: event.scheduled_departure,
                        status: if cancelled {
                            TripStatus::Cancelled
                        } else if let Some(t) = event.realtime_departure {
                            TripStatus::Realtime(t)
                        } else {
                            TripStatus::NoRealtime
                        },
                    }
                })
                .collect::<Vec<_>>();

            trips.sort();
            LineStatus {
                settings: line_clone,
                trips,
            }
        }
    });
    let mut ret = join_all(futures)
        .await
        .into_iter()
        .filter(|status| !status.trips.is_empty())
        .collect::<Vec<_>>();
    ret.sort_by(|a, b| {
        a.settings
            .priority
            .cmp(&b.settings.priority)
            .then(a.trips[0].cmp(&b.trips[1]))
    });
    Ok(ret)
}

async fn fetch_stop_status(
    stop_id: &str,
    chateau_id: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<StopApiStatus, reqwest::Error> {
    let start = start.timestamp();
    let end = end.timestamp();
    let url = format!("https://birchdeparturesfromstop.catenarymaps.org/departures_at_stop?stop_id={stop_id}&chateau_id={chateau_id}&greater_than_time={start}&less_than_time={end}&include_shapes=false");
    reqwest::get(&url).await?.json().await
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Schedule(params: String) -> Element {
    let url_decoded = urlencoding::decode(&params)?;
    let settings: Settings = ron::from_str(&url_decoded)?;

    let mut time = use_signal(Utc::now);
    use_future(move || async move {
        loop {
            time.set(Utc::now());
            async_std::task::sleep(UPDATE_DELTA).await;
        }
    })();

    let mut results = use_signal(|| None);
    let settings_clone = settings.clone();
    use_future(move || {
        let lines = settings_clone.lines.clone();
        async move {
            loop {
                let time = *time.read();
                if let Ok(res) = fetch_stops(
                    &lines.clone(),
                    time,
                    time + TimeDelta::seconds(settings.max_ahead as i64),
                )
                .await
                {
                    results.set(Some(res));
                }
                async_std::task::sleep(RELOAD_DURATION).await;
            }
        }
    })();

    rsx! {
        div {
            class: "header",
            div {
                class: "title",
                { settings.title }
            }
            div {
                class: "time",
                { time.read().with_timezone(&Local).format(TIME_FORMAT).to_string() }
            }
        }
        if let Some(res) = &*results.read() {
            div {
                class: "lines",
                for line in res {
                    LineDisplay { line: line.clone(), time }
                }
            }
        }
    }
}

#[component]
fn LineDisplay(line: LineStatus, time: Signal<DateTime<Utc>>) -> Element {
    rsx! {
        div {
            class: "line",
            div {
                class: "line-info",
                div {
                    class: "line-number",
                    color: line.settings.color.clone(),
                    background_color: line.settings.background_color.clone(),
                    border_color: if let Some(col) = line.settings.outline_color {
                        Some(col.clone())
                    } else {
                        line.settings.background_color.clone()
                    },
                    { line.settings.id }
                }
                if let Some(label) = line.settings.label {
                    div {
                        class: "line-label",
                        span {
                            class: "line-label-scroll",
                            { label }
                        }
                    }
                }
            }

            div {
                class: "line-departures",
                for trip in line.trips.iter().take(MAX_TRIPS_SHOWN) {
                    div {
                        class: match trip.status {
                            TripStatus::Cancelled => "trip trip-cancelled",
                            TripStatus::Realtime(_) => "trip trip-realtime",
                            TripStatus::NoRealtime => "trip"
                        },
                        {
                            let ts = DateTime::<Utc>::from_timestamp_secs(trip.scheduled).ok_or(anyhow!("Error converting timestamp"))?;
                            match trip.status {
                                TripStatus::Cancelled | TripStatus::NoRealtime => rsx!{
                                    {ts.with_timezone(&Local).format(TIME_FORMAT).to_string()}
                                },
                                TripStatus::Realtime(t) => rsx!{
                                    div {
                                        class: "trip-content",
                                            {format!("{} min",
                                            (DateTime::<Utc>::from_timestamp_secs(t).ok_or(anyhow!("Error converting realtime timestamp"))?
                                            -*time.read()).num_minutes())}
                                        object {
                                            type: "image/svg+xml",
                                            data: LIVE_SVG,
                                            class: "live-icon",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
