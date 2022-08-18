use maud::html;
use polyester::browser;
use polyester::browser::DomId;
use polyester::browser::Effects;
use polyester::browser::SubscriptionMsg;
use polyester::browser::ToDomId;
use polyester::page::Page;
use polyester::page::PageMarkup;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub count: isize,
}

pub struct HomePage {}

impl Page<Model, Msg, CustomEffect> for HomePage {
    fn id(&self) -> DomId {
        DomId::new("sevenguis")
    }

    fn init(&self) -> (Model, Effects<Msg, CustomEffect>) {
        let model = Model { count: 0 };

        let effects = vec![];

        (model, effects)
    }

    fn subscriptions(&self, _model: &Model) -> browser::Subscriptions<Msg, CustomEffect> {
        vec![
            browser::on_click(&Id::Increment.to_dom_id(), Msg::Increment),
            browser::on_click(&Id::Decrement.to_dom_id(), Msg::Decrement),
        ]
    }

    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effects<Msg, CustomEffect>, String> {
        match msg {
            Msg::Increment => {
                model.count += 1;
                Ok(vec![])
            }

            Msg::Decrement => {
                model.count -= 1;
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
    Increment,
    Decrement,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Msg {
    Increment,
    Decrement,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CustomEffect {}

fn view_head() -> maud::Markup {
    html! {
        title { "Home page" }
        link rel="stylesheet" href="./app.css";
        script defer type="module" src="./home_page.js" {}
    }
}

fn view_body(page_id: &browser::DomId, _model: &Model) -> maud::Markup {
    html! {
        div id=(page_id) class="p-4" {
            h1 class="text-xl text-bold" { "7GUIs demos" }

            ul class="mt-4" {
                li {
                    a class="underline text-blue-600 hover:text-blue-800 visited:text-purple-600" href="/counter.html" {
                        "Counter"
                    }
                }

                li {
                    a class="underline text-blue-600 hover:text-blue-800 visited:text-purple-600" href="/temperature.html" {
                        "Temperature Converter"
                    }
                }

                li {
                    a class="underline text-blue-600 hover:text-blue-800 visited:text-purple-600" href="/flight.html" {
                        "Flight Booker"
                    }
                }

                li {
                    a class="underline text-blue-600 hover:text-blue-800 visited:text-purple-600" href="/timer.html" {
                        "Timer"
                    }
                }

                li {
                    a class="underline text-blue-600 hover:text-blue-800 visited:text-purple-600" href="/crud.html" {
                        "CRUD"
                    }
                }
            }
        }
    }
}
