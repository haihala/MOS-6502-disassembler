use askama::Template;
use poem::web::{Form, Multipart};
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

    #[oai(path = "/table", method = "post")]
    pub async fn table(&self, Form(params): Form<TableParams>) -> Html<String> {
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
            let lines = disassemble(&bytes);
            Html(TableTemplate { lines }.render().unwrap())
        }
    }

    #[oai(path = "/decode", method = "post")]
    pub async fn decode_file(&self, mut multipart: Multipart) -> Html<String> {
        let Ok(Some(file)) = multipart.next_field().await else {
            // No file = empty file
            return Html(String::new());
        };

        Html(
            file.bytes()
                .await
                .unwrap_or_default() // Failed to parse file = empty file
                .into_iter()
                .map(|byte| format!("{:0>2X}", byte))
                .collect::<Vec<_>>()
                .chunks(8)
                .map(|chunk| chunk.join(" "))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_table_generation() {
        let client = reqwest::Client::new();

        let output = client
            .post("http://localhost:9999/table")
            .form(
                &vec![("bytes", "a9 bd a0 bd 20 28 ba")]
                    .into_iter()
                    .collect::<HashMap<&str, &str>>(),
            )
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        for chunks in [
            ["0000", "A9 BD", "LDA", "#$BD"],
            ["0002", "A0 BD", "LDY", "#$BD"],
            ["0004", "20 28 BA", "JSR", "$BA28"],
        ] {
            dbg!(&chunks, &output);
            for chunk in chunks {
                assert!(
                    output.contains(chunk),
                    "output: {}, chunk: {}",
                    output,
                    chunk
                );
            }
        }
    }

    #[tokio::test]
    async fn test_faulty_table() {
        let client = reqwest::Client::new();

        let output = client
            .post("http://localhost:9999/table")
            .form(
                &vec![("bytes", "abcdefgh")]
                    .into_iter()
                    .collect::<HashMap<&str, &str>>(),
            )
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert!(output.contains("'gh' at byte 3"), "output: {}", output);
    }

    #[tokio::test]
    async fn test_decode() {
        const URL: &str = "http://localhost:9999/decode";
        let client = reqwest::Client::new();

        let bytes = std::fs::read("test-bin/test1.bin").unwrap();

        let lines: String = client
            .post(URL)
            .multipart(
                reqwest::multipart::Form::new()
                    .part("file", reqwest::multipart::Part::stream(bytes)),
            )
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let expected = r#"48 E7 20 20 70 21 61 00
F8 EE 61 E6 61 00 04 02
22 6E 00 84 41 E9 00 16
74 07 0C 00 00 44 67 18
41 E8 00 20 74 06 0C 00
00 41 67 0C 45 E9 00 06
0C 00 00 55 67 1E 60 38"#
            .to_string();

        assert_eq!(expected, lines);
    }
}
