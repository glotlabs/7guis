use chrono::NaiveDate;
use maud::html;
use polyester::browser;
use polyester::browser::to_value;
use polyester::browser::DomId;
use polyester::browser::Effects;
use polyester::browser::ToDomId;
use polyester::browser::Value;
use polyester::page::Page;
use polyester::page::PageMarkup;
use polyester::time;
use polyester::time::Posix;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Model {
    Drafting(Draft),
    Booked(Flight),
}

pub struct FlightPage {
    pub initial_time: time::Posix,
}

impl FlightPage {
    fn initial_model(&self) -> Model {
        let now = posix_to_naive_date_time(&self.initial_time);

        Model::Drafting(Draft {
            flight_type: FlightType::OneWay,
            current_date: now.date(),
            start_date: Date::Valid(now.date()),
            return_date: Date::Valid(now.date()),
        })
    }
}

impl Page<Model, Msg, AppEffect> for FlightPage {
    fn id(&self) -> DomId {
        DomId::new("sevenguis")
    }

    fn init(&self) -> (Model, Effects<Msg, AppEffect>) {
        let model = self.initial_model();
        let effects = vec![];

        (model, effects)
    }

    fn subscriptions(&self, model: &Model) -> browser::Subscriptions<Msg, AppEffect> {
        match model {
            Model::Drafting(_) => {
                vec![
                    browser::on_change(&Id::FlightType, Msg::FlightTypeChanged),
                    browser::on_change_string(&Id::StartDate, Msg::StartDateChanged),
                    browser::on_change_string(&Id::ReturnDate, Msg::ReturnDateChanged),
                    browser::on_click(&Id::Book, Msg::BookFlight),
                ]
            }

            Model::Booked(_) => {
                vec![browser::on_click(&Id::Reset, Msg::Reset)]
            }
        }
    }

    fn update(&self, msg: &Msg, mut model: &mut Model) -> Result<Effects<Msg, AppEffect>, String> {
        match (&mut model, msg) {
            (Model::Drafting(draft), Msg::FlightTypeChanged(value)) => {
                let flight_type = value
                    .parse()
                    .map_err(|err| format!("Failed to parse flight type: {}", err))?;

                draft.flight_type = flight_type;

                Ok(vec![])
            }

            (Model::Drafting(draft), Msg::StartDateChanged(value)) => {
                draft.start_date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
                    .map(Date::Valid)
                    .unwrap_or_else(|_| Date::Invalid(value.to_string()));

                Ok(vec![])
            }

            (Model::Drafting(draft), Msg::ReturnDateChanged(value)) => {
                draft.return_date = NaiveDate::parse_from_str(value, "%Y-%m-%d")
                    .map(Date::Valid)
                    .unwrap_or_else(|_| Date::Invalid(value.to_string()));

                Ok(vec![])
            }

            (Model::Drafting(draft), Msg::BookFlight) => {
                let flight = Flight::from_draft(draft).ok_or("Failed to book flight")?;
                *model = Model::Booked(flight);

                Ok(vec![])
            }

            (_, Msg::Reset) => {
                let new_model = self.initial_model();
                *model = new_model;

                Ok(vec![])
            }

            (Model::Booked(_), _) => Ok(vec![]),
        }
    }

    fn view(&self, model: &Model) -> PageMarkup {
        PageMarkup {
            head: view_head(),
            body: view_body(&self.id(), model),
        }
    }
}

#[derive(strum_macros::Display, polyester_macro::ToDomId)]
#[strum(serialize_all = "kebab-case")]
enum Id {
    FlightType,
    StartDate,
    ReturnDate,
    Book,
    Reset,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Msg {
    FlightTypeChanged(Value),
    StartDateChanged(String),
    ReturnDateChanged(String),
    BookFlight,
    Reset,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppEffect {}

fn view_head() -> maud::Markup {
    html! {
        title { "Flight booker" }
        link rel="stylesheet" href="/app.css";
        script defer type="module" src="/flight_page.js" {}
    }
}

fn view_body(page_id: &DomId, model: &Model) -> maud::Markup {
    html! {
        div id=(page_id) class="p-4" {
            @match model {
                Model::Drafting(draft) => {
                    (view_form(draft))
                },

                Model::Booked(flight) => {
                    (view_success(flight))
                },
            }
        }
    }
}

fn view_form(draft: &Draft) -> maud::Markup {
    let flight_type = &draft.flight_type;
    let start_date_is_ok = draft.start_date_is_ok(&draft.current_date);
    let return_date_is_ok = draft.return_date_is_ok(&draft.current_date);
    let can_book = start_date_is_ok && return_date_is_ok;

    html! {
        div {
            label for=(Id::FlightType) { "Flight type" }
            div {
                select id=(Id::FlightType) {
                    option value=(to_value(FlightType::OneWay)) selected[flight_type.is_one_way()] {
                        "One-way flight"
                    }
                    option value=(to_value(FlightType::Return)) selected[!flight_type.is_one_way()] {
                        "Return flight"
                    }
                }
            }
        }

        div class="mt-4"{
            label for=(Id::StartDate) { "Start date" }
            div {
                input id=(Id::StartDate) type="date" style=(conditional(!start_date_is_ok, "background: coral;")) value=(draft.start_date);
            }
        }

        div class="mt-4" {
            label for=(Id::ReturnDate) { "End date" }
            div {
                input id=(Id::ReturnDate) type="date" style=(conditional(!return_date_is_ok, "background: coral;")) value=(draft.return_date) disabled[!draft.return_date_is_needed()];
            }
        }

        div class="mt-4"{
            button id=(Id::Book) disabled[!can_book] class="text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                "Book"
            }
        }
    }
}

fn view_success(flight: &Flight) -> maud::Markup {
    let fmt = "%d.%m.%Y";
    let message = match flight {
        Flight::OneWay { start_date } => {
            format!(
                "You have booked a one-way flight on {}.",
                start_date.format(fmt)
            )
        }

        Flight::Return {
            start_date,
            return_date,
        } => {
            format!(
                "You have booked a return flight from {} to {}.",
                start_date.format(fmt),
                return_date.format(fmt)
            )
        }
    };

    html! {
        div { (message) }
        button id=(Id::Reset) class="mt-4 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
            "Book another flight"
        }

    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Draft {
    current_date: NaiveDate,
    flight_type: FlightType,
    start_date: Date,
    return_date: Date,
}

impl Draft {
    fn start_date_is_ok(&self, current_date: &NaiveDate) -> bool {
        match self.start_date.date() {
            Some(start_date) => start_date >= *current_date,
            None => false,
        }
    }

    fn return_date_is_ok(&self, current_date: &NaiveDate) -> bool {
        if !self.return_date_is_needed() {
            true
        } else {
            match (self.start_date.date(), self.return_date.date()) {
                (Some(start_date), Some(return_date)) => {
                    return_date >= start_date && return_date >= *current_date
                }

                _ => false,
            }
        }
    }

    fn return_date_is_needed(&self) -> bool {
        !self.flight_type.is_one_way()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum Flight {
    OneWay {
        start_date: NaiveDate,
    },
    Return {
        start_date: NaiveDate,
        return_date: NaiveDate,
    },
}

impl Flight {
    fn from_draft(draft: &Draft) -> Option<Flight> {
        match draft.flight_type {
            FlightType::OneWay => draft
                .start_date
                .date()
                .map(|start_date| Flight::OneWay { start_date }),

            FlightType::Return => draft.start_date.date().zip(draft.return_date.date()).map(
                |(start_date, return_date)| Flight::Return {
                    start_date,
                    return_date,
                },
            ),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
enum Date {
    Valid(NaiveDate),
    Invalid(String),
}

impl Date {
    pub fn date(&self) -> Option<NaiveDate> {
        match self {
            Date::Valid(date) => Some(date.clone()),

            Date::Invalid(_) => None,
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Date::Valid(date) => {
                write!(f, "{}", date.format("%Y-%m-%d"))
            }

            Date::Invalid(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum FlightType {
    OneWay,
    Return,
}

impl FlightType {
    pub fn is_one_way(&self) -> bool {
        self == &FlightType::OneWay
    }
}

fn conditional<T>(should_render: bool, value: T) -> Optional<T> {
    Optional(should_render.then(|| value))
}

struct Optional<T>(Option<T>);

impl<T: maud::Render> maud::Render for Optional<T> {
    fn render_to(&self, buffer: &mut String) {
        if let Some(ref value) = self.0 {
            value.render_to(buffer);
        }
    }
}

fn posix_to_naive_date_time(posix: &Posix) -> chrono::NaiveDateTime {
    let ns = (posix.as_millis() % 1000) * 1_000_000;
    chrono::NaiveDateTime::from_timestamp(posix.as_secs() as i64, ns.try_into().unwrap_or(0))
}
