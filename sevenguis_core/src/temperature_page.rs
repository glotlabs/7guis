use maud::html;
use polyester::browser;
use polyester::browser::DomId;
use polyester::browser::Effects;
use polyester::browser::ToDomId;
use polyester::page::Page;
use polyester::page::PageMarkup;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub celsius: Option<i32>,
    pub fahrenheit: Option<i32>,
}

pub struct TemperaturePage {}

impl Page<Model, Msg, AppEffect> for TemperaturePage {
    fn id(&self) -> DomId {
        DomId::new("sevenguis")
    }

    fn init(&self) -> (Model, Effects<Msg, AppEffect>) {
        let model = Model {
            celsius: None,
            fahrenheit: None,
        };

        let effects = vec![];

        (model, effects)
    }

    fn subscriptions(&self, _model: &Model) -> browser::Subscriptions<Msg, AppEffect> {
        vec![
            browser::on_input(&Id::Celsius, Msg::CelsiusChanged),
            browser::on_input(&Id::Fahrenheit, Msg::FahrenheitChanged),
        ]
    }

    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effects<Msg, AppEffect>, String> {
        match msg {
            Msg::CelsiusChanged(value) => {
                let str_value: String = value
                    .parse()
                    .map_err(|err| format!("Failed to parse celsius as string: {}", err))?;

                match str_value.parse() {
                    Ok(celsius) => {
                        model.celsius = Some(celsius);
                        model.fahrenheit = Some(celsius_to_fahrenheit(celsius));
                    }

                    Err(_) => model.fahrenheit = None,
                }

                Ok(vec![])
            }

            Msg::FahrenheitChanged(value) => {
                let str_value: String = value
                    .parse()
                    .map_err(|err| format!("Failed to parse celsius as string: {}", err))?;

                match str_value.parse() {
                    Ok(fahrenheit) => {
                        model.fahrenheit = Some(fahrenheit);
                        model.celsius = Some(fahrenheit_to_celsius(fahrenheit));
                    }

                    Err(_) => model.celsius = None,
                }

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

#[derive(strum_macros::Display, polyester_macro::ToDomId)]
#[strum(serialize_all = "kebab-case")]
enum Id {
    Celsius,
    Fahrenheit,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Msg {
    CelsiusChanged(String),
    FahrenheitChanged(String),
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppEffect {}

fn view_head() -> maud::Markup {
    html! {
        title { "Temperature converter - polyester" }
        link rel="stylesheet" href="/app.css";
        script defer type="module" src="/temperature_page.js" {}
    }
}

fn view_body(page_id: &browser::DomId, model: &Model) -> maud::Markup {
    let celsius = model.celsius.map(|n| n.to_string()).unwrap_or_default();
    let fahrenheit = model.fahrenheit.map(|n| n.to_string()).unwrap_or_default();

    html! {
        div id=(page_id) {
            div class="flex p-4" {
                div {
                    label class="block text-sm font-medium text-gray-700" for=(Id::Celsius) {
                        "Celsius"
                    }
                    div class="mt-1" {
                        input id=(Id::Celsius) value=(celsius) class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" type="number";
                    }
                }

                div class="ml-4"{
                    label class="block text-sm font-medium text-gray-700" for=(Id::Fahrenheit) {
                        "Fahrenheit"
                    }
                    div class="mt-1" {
                        input id=(Id::Fahrenheit) value=(fahrenheit) class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" type="number";
                    }
                }
            }
        }
    }
}

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    let fahrenheit = f64::from(celsius) * (9.0 / 5.0) + 32.0;
    fahrenheit.round() as i32
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    let celsius = (f64::from(fahrenheit) - 32.0) * (5.0 / 9.0);
    celsius.round() as i32
}
