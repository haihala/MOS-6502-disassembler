use askama::Template;
use poem::web::{Multipart, Query};
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
struct TableErrorTemplate {
    illegals: Vec<(usize, String)>,
}

pub struct Frontend;
#[OpenApi]
impl Frontend {
    #[oai(path = "/", method = "get")]
    pub async fn front_page(&self) -> Html<String> {
        Html(MainPage.render().unwrap())
    }

    #[oai(path = "/table", method = "get")]
    pub async fn table(&self, params: Query<TableParams>) -> Html<String> {
        let mut illegals = vec![];

        let bytes = params
            .bytes
            .chars()
            .filter(|c| !c.is_ascii_whitespace()) // Strip whitespace
            // Form into pairs (2 hexadecimal digits -> 1 byte)
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
            .enumerate()
            .filter_map(|(index, c)| {
                if let Ok(digit) = u8::from_str_radix(c.as_str(), 16) {
                    Some(digit)
                } else {
                    illegals.push((index, c));
                    None
                }
            })
            .collect::<Vec<u8>>();

        if !illegals.is_empty() {
            Html(TableErrorTemplate { illegals }.render().unwrap())
        } else {
            let lines = disassemble(bytes);
            Html(TableTemplate { lines }.render().unwrap())
        }
    }

    #[oai(path = "/decode", method = "post")]
    pub async fn file(&self, mut multipart: Multipart) -> Html<String> {
        let Ok(Some(file)) = multipart.next_field().await else {
            return Html("Could not decode, no file provided".to_string());
        };

        let Ok(bytes) = file.bytes().await else {
            return Html("Provided file is empty".to_string());
        };

        Html(
            bytes
                .into_iter()
                .map(|byte| format!("{:0<2x}", byte))
                .collect::<Vec<_>>()
                .chunks(8)
                .map(|chunk| chunk.join(" "))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
