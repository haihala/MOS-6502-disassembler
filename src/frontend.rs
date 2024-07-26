use askama::Template;
use poem::web::Query;
use poem_openapi::{payload::Html, OpenApi};
use serde::Deserialize;

use crate::{disassemble, StructuredInstruction};

#[derive(Template)]
#[template(path = "main.html")]
struct MainPage;

#[derive(Debug, Deserialize)]
pub struct TableParams {
    bytes: String,
}

#[derive(Template)]
#[template(path = "table.html")]
struct TableTemplate {
    lines: Vec<StructuredInstruction>,
}

#[derive(Template)]
#[template(path = "table-error.html")]
struct TableErrorTemplate;

pub struct Frontend;
#[OpenApi]
impl Frontend {
    #[oai(path = "/", method = "get")]
    pub async fn front_page(&self) -> Html<String> {
        Html(MainPage.render().unwrap())
    }

    #[oai(path = "/table", method = "get")]
    pub async fn table(&self, params: Query<TableParams>) -> Html<String> {
        let filtered_input = params
            .bytes
            .chars()
            // This filters out newlines and spaces for convenience
            .filter(|&c| c.is_ascii_alphanumeric())
            .collect::<String>();
        let Ok(bytes) = filtered_input
            .chars()
            .fold(vec![], |mut acc: Vec<String>, incoming| {
                match acc.last_mut() {
                    Some(s) if s.len() == 1 => {
                        *s = format!("{}{}", s, incoming);
                    }
                    _ => {
                        acc.push(incoming.to_string());
                    }
                };
                acc
            })
            .into_iter()
            .map(|c| u8::from_str_radix(c.as_str(), 16))
            .collect::<Result<Vec<u8>, _>>()
        else {
            return Html(TableErrorTemplate.render().unwrap());
        };
        let lines = disassemble(bytes);
        Html(TableTemplate { lines }.render().unwrap())
    }
}
