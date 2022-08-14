use maud::html;
use polyester::browser;
use polyester::browser::DomId;
use polyester::browser::Effects;
use polyester::browser::SubscriptionMsg;
use polyester::page::Page;
use polyester::page::PageMarkup;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub ids: DomIds,
    pub count: isize,
}

pub struct HomePage {}

impl Page<Model, Msg, CustomEffect> for HomePage {
    fn id(&self) -> DomId {
        DomId::new("sevenguis")
    }

    fn init(&self) -> (Model, Effects<Msg, CustomEffect>) {
        let model = Model {
            ids: initial_ids(),
            count: 0,
        };

        let effects = vec![];

        (model, effects)
    }

    fn subscriptions(&self, model: &Model) -> browser::Subscriptions<Msg, CustomEffect> {
        vec![
            browser::on_click(&model.ids.increment, SubscriptionMsg::pure(Msg::Increment)),
            browser::on_click(&model.ids.decrement, SubscriptionMsg::pure(Msg::Decrement)),
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Msg {
    Increment,
    Decrement,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CustomEffect {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomIds {
    pub increment: DomId,
    pub decrement: DomId,
}

fn initial_ids() -> DomIds {
    DomIds {
        increment: "increment".into(),
        decrement: "decrement".into(),
    }
}

fn view_head() -> maud::Markup {
    html! {
        title { "Home page" }
        link rel="stylesheet" href="./app.css";
        script async type="module" src="./home_page.js" {}
    }
}

fn view_body(page_id: &browser::DomId, model: &Model) -> maud::Markup {
    html! {
        div id=(page_id) {
            div class="flex p-4" {
                button id=(model.ids.decrement) class="w-28 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                    "Decrement"
                }
                div class="mx-4 w-28" {
                    input value=(model.count) class="text-center shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" type="text" readonly;
                }
                button id=(model.ids.increment) class="w-28 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                    "Increment"
                }
            }
        }
    }
}
