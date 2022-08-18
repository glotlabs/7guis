use maud::html;
use polyester::browser;
use polyester::browser::DomId;
use polyester::browser::Effects;
use polyester::browser::ToDomId;
use polyester::browser::Value;
use polyester::page::Page;
use polyester::page::PageMarkup;
use serde::{Deserialize, Serialize};

#[derive(strum_macros::Display, polyester_macro::ToDomId)]
#[strum(serialize_all = "kebab-case")]
enum Id {
    Filter,
    FirstName,
    LastName,
    People,
    Create,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub next_id: u32,
    pub selected_id: u32,
    pub people: Vec<Person>,
    pub form: Form,
    pub filter: String,
}

pub struct CrudPage {}

impl Page<Model, Msg, CustomEffect> for CrudPage {
    fn id(&self) -> DomId {
        DomId::new("sevenguis")
    }

    fn init(&self) -> (Model, Effects<Msg, CustomEffect>) {
        let model = Model {
            next_id: 4,
            selected_id: 1,
            people: vec![
                Person {
                    id: 1,
                    first_name: "Hans".to_string(),
                    last_name: "Emil".to_string(),
                },
                Person {
                    id: 2,
                    first_name: "Max".to_string(),
                    last_name: "Mustermann".to_string(),
                },
                Person {
                    id: 3,
                    first_name: "Roman".to_string(),
                    last_name: "Tisch".to_string(),
                },
            ],
            form: Form {
                first_name: "".to_string(),
                last_name: "".to_string(),
            },
            filter: "".to_string(),
        };

        let effects = vec![];

        (model, effects)
    }

    fn subscriptions(&self, _model: &Model) -> browser::Subscriptions<Msg, CustomEffect> {
        vec![
            browser::on_input(&Id::Filter.to_dom_id(), Msg::FilterChanged),
            browser::on_input(&Id::FirstName.to_dom_id(), Msg::FirstNameChanged),
            browser::on_input(&Id::LastName.to_dom_id(), Msg::LastNameChanged),
            browser::on_change(&Id::People.to_dom_id(), Msg::SelectedPersonChanged),
            browser::on_click(&Id::Create.to_dom_id(), Msg::Create),
            browser::on_click(&Id::Update.to_dom_id(), Msg::Update),
            browser::on_click(&Id::Delete.to_dom_id(), Msg::Delete),
        ]
    }

    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effects<Msg, CustomEffect>, String> {
        match msg {
            Msg::FilterChanged(value) => {
                let filter: String = value
                    .parse()
                    .map_err(|err| format!("Failed to parse filter: {}", err))?;

                model.filter = filter.to_lowercase();
                Ok(vec![])
            }

            Msg::SelectedPersonChanged(value) => {
                let selected_id = value
                    .parse()
                    .map_err(|err| format!("Failed to parse selected person: {}", err))?;

                model.selected_id = selected_id;

                Ok(vec![])
            }

            Msg::FirstNameChanged(value) => {
                let first_name = value
                    .parse()
                    .map_err(|err| format!("Failed to parse first name: {}", err))?;

                model.form.first_name = first_name;

                Ok(vec![])
            }

            Msg::LastNameChanged(value) => {
                let last_name = value
                    .parse()
                    .map_err(|err| format!("Failed to parse last name: {}", err))?;

                model.form.last_name = last_name;

                Ok(vec![])
            }

            Msg::Create => {
                model.people.push(Person {
                    id: model.next_id,
                    first_name: model.form.first_name.clone(),
                    last_name: model.form.last_name.clone(),
                });

                model.form = Form::empty();
                model.selected_id = model.next_id;
                model.next_id += 1;

                Ok(vec![])
            }

            Msg::Update => {
                model
                    .people
                    .iter_mut()
                    .find(|person| person.id == model.selected_id)
                    .map(|person| {
                        person.first_name = model.form.first_name.clone();
                        person.last_name = model.form.last_name.clone();
                    });

                model.form = Form::empty();

                Ok(vec![])
            }

            Msg::Delete => {
                model.people.retain(|p| p.id != model.selected_id);
                model.selected_id = model.people.first().map(|p| p.id).unwrap_or(0);

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
    FilterChanged(String),
    SelectedPersonChanged(Value),
    FirstNameChanged(String),
    LastNameChanged(String),
    Create,
    Update,
    Delete,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CustomEffect {}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Person {
    id: u32,
    first_name: String,
    last_name: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Form {
    first_name: String,
    last_name: String,
}

impl Form {
    pub fn empty() -> Form {
        Form {
            first_name: "".to_string(),
            last_name: "".to_string(),
        }
    }
}

fn view_head() -> maud::Markup {
    html! {
        title { "Crud" }
        link rel="stylesheet" href="./app.css";
        script defer type="module" src="./crud_page.js" {}
    }
}

fn view_body(page_id: &browser::DomId, model: &Model) -> maud::Markup {
    let filtered_people = model
        .people
        .iter()
        .filter(|person| {
            model.filter.is_empty() || person.last_name.to_lowercase().starts_with(&model.filter)
        })
        .collect::<Vec<&Person>>();

    html! {
        div id=(page_id) class="p-4" {
            div {
                label {
                    div { "Filter" }
                    input id=(Id::Filter) value=(model.filter) type="text";
                }
            }

            select id=(Id::People) size="5" class="mt-4" {
                @for person in filtered_people {
                    option value=(person.id) selected[person.id == model.selected_id] {
                        (format!("{}, {}", person.last_name, person.first_name))
                    }
                }
            }

            div class="mt-4" {
                label {
                    div { "First name" }
                    input id=(Id::FirstName) value=(model.form.first_name) type="text";
                }
            }

            div class="mt-4" {
                label {
                    div { "Last name" }
                    input id=(Id::LastName) value=(model.form.last_name) type="text";
                }
            }

            div class="mt-4" {
                button id=(Id::Create) class="w-20 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                    "Create"
                }
                button id=(Id::Update) class="w-20 ml-4 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                    "Update"
                }
                button id=(Id::Delete) class="w-20 ml-4 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                    "Delete"
                }
            }
        }
    }
}
