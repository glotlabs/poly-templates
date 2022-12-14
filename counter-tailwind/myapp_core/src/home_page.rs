use maud::html;
use maud::Markup;
use poly::browser;
use poly::browser::DomId;
use poly::browser::Effects;
use poly::page;
use poly::page::Page;
use poly::page::PageMarkup;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub count: isize,
}

pub struct HomePage {
    pub current_url: Url,
}

impl Page<Model, Msg, AppEffect, Markup> for HomePage {
    fn id(&self) -> &'static dyn DomId {
        &Id::MyApp
    }

    fn init(&self) -> Result<(Model, Effects<Msg, AppEffect>), String> {
        let model = Model { count: 0 };

        let effects = vec![];

        Ok((model, effects))
    }

    fn subscriptions(&self, _model: &Model) -> browser::Subscriptions<Msg, AppEffect> {
        vec![
            browser::on_click(Id::Increment, Msg::Increment),
            browser::on_click(Id::Decrement, Msg::Decrement),
        ]
    }

    fn update(&self, msg: &Msg, model: &mut Model) -> Result<Effects<Msg, AppEffect>, String> {
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

    fn view(&self, model: &Model) -> PageMarkup<Markup> {
        PageMarkup {
            head: view_head(),
            body: view_body(model),
        }
    }

    fn render(&self, markup: Markup) -> String {
        markup.into_string()
    }

    fn render_page(&self, markup: PageMarkup<Markup>) -> String {
        page::render_page_maud(markup)
    }
}

#[derive(strum_macros::Display, poly_macro::DomId)]
#[strum(serialize_all = "kebab-case")]
enum Id {
    MyApp,
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
pub enum AppEffect {}

fn view_head() -> maud::Markup {
    html! {
        title { "Home Page" }
        meta name="viewport" content="width=device-width, initial-scale=1";
        link rel="stylesheet" href="/app.css";
        script defer type="module" src="/home_page.js" {}
    }
}

fn view_body(model: &Model) -> maud::Markup {
    html! {
        div id=(Id::MyApp) {
            div class="flex p-4" {
                button id=(Id::Decrement) class="w-28 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                    "Decrement"
                }
                div class="mx-4 w-28" {
                    input value=(model.count) class="text-center shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" type="text" readonly;
                }
                button id=(Id::Increment) class="w-28 text-center items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button" {
                    "Increment"
                }
            }
        }
    }
}
