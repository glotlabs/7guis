use maud::html;
use polyester::browser;
use polyester::browser::dom;
use polyester::browser::time as time_effect;
use polyester::browser::DomId;
use polyester::browser::Effects;
use polyester::browser::SubscriptionMsg;
use polyester::browser::ToDomId;
use polyester::browser::Value;
use polyester::page::Page;
use polyester::page::PageMarkup;
use polyester::time;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub current_time: time::Posix,
    pub previous_time: time::Posix,
    pub max_duration: std::time::Duration,
    pub elapsed: std::time::Duration,
}

pub struct TimerPage {
    pub initial_time: time::Posix,
}

impl Page<Model, Msg, CustomEffect> for TimerPage {
    fn id(&self) -> DomId {
        DomId::new("sevenguis")
    }

    fn init(&self) -> (Model, Effects<Msg, CustomEffect>) {
        let model = Model {
            current_time: self.initial_time,
            previous_time: self.initial_time,
            max_duration: Duration::from_secs(15),
            elapsed: Duration::from_secs(0),
        };

        let effects = vec![];

        (model, effects)
    }

    fn subscriptions(&self, model: &Model) -> browser::Subscriptions<Msg, CustomEffect> {
        vec![
            browser::on_input(&Id::Duration.to_dom_id(), Msg::MaxDurationChanged),
            browser::on_click(&Id::Reset.to_dom_id(), Msg::ResetClicked),
            browser::interval_effect(
                Duration::from_millis(100),
                Msg::GotTime,
                time_effect::current_time(),
            ),
            if model.elapsed < model.max_duration {
                browser::interval_effect(
                    Duration::from_millis(200),
                    Msg::OnTick,
                    time_effect::current_time(),
                )
            } else {
                browser::no_subscription()
            },
        ]
    }

    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effects<Msg, CustomEffect>, String> {
        match msg {
            Msg::MaxDurationChanged(value) => {
                if model.elapsed >= model.max_duration {
                    model.previous_time = model.current_time;
                }

                let max_duration: u64 = value
                    .parse()
                    .map_err(|err| format!("Failed to parse max duration: {}", err))?;

                model.max_duration = Duration::from_millis(max_duration);
                if model.elapsed > model.max_duration {
                    model.elapsed = model.max_duration
                }

                Ok(vec![])
            }

            Msg::GotTime(value) => {
                model.current_time = parse_current_time(value)?;
                Ok(vec![])
            }

            Msg::OnTick(value) => {
                model.current_time = parse_current_time(value)?;

                let elapsed = model.elapsed + (model.current_time - model.previous_time);

                if elapsed > model.max_duration {
                    model.elapsed = model.max_duration
                } else {
                    model.elapsed = elapsed;
                }

                model.previous_time = model.current_time;

                Ok(vec![])
            }

            Msg::ResetClicked => {
                model.elapsed = Duration::from_secs(0);
                model.previous_time = model.current_time;

                Ok(vec![])
            }
        }
    }

    fn view(&self, model: &Model) -> PageMarkup {
        PageMarkup {
            head: view_head(),
            body: view_body(&self.id(), model),
        }
    }
}

fn parse_current_time(value: &Value) -> Result<time::Posix, String> {
    let current_time = value
        .parse()
        .map_err(|err| format!("Failed to parse current time: {}", err))?;

    Ok(current_time)
}

#[derive(strum_macros::Display, polyester_macro::ToDomId)]
#[strum(serialize_all = "kebab-case")]
enum Id {
    Elapsed,
    Duration,
    Reset,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Msg {
    GotTime(Value),
    MaxDurationChanged(String),
    OnTick(Value),
    ResetClicked,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CustomEffect {}

fn view_head() -> maud::Markup {
    html! {
        title { "Timer" }
        link rel="stylesheet" href="./app.css";
        script defer type="module" src="./timer_page.js" {}
    }
}

fn view_body(page_id: &browser::DomId, model: &Model) -> maud::Markup {
    let max_elapsed = model.max_duration.as_millis();
    let elapsed = model.elapsed.as_millis();
    let elapsed_text = format!(
        "{:.2}s / {:.2}s",
        model.elapsed.as_secs_f64(),
        model.max_duration.as_secs_f64()
    );
    let max_duration = model.max_duration.as_millis();

    html! {
        div id=(page_id) {
            div class="p-4" {
                div {
                    label for=(Id::Elapsed) {
                        "Elapsed time"
                    }
                    div {
                        meter id=(Id::Elapsed) min="0" max=(max_elapsed) value=(elapsed) class="w-32" {}
                    }
                    div {
                        (elapsed_text)
                    }
                }

                div class="mt-4" {
                    label for=(Id::Duration) {
                        "Duration"
                    }
                    div {
                        input id=(Id::Duration) type="range" min="0" max="30000" value=(max_duration) class="w-32";
                    }
                }

                button id=(Id::Reset) class="mt-4 w-32 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                    "Reset"
                }
            }
        }
    }
}
