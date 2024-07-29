use askama::Template;
use poem::web::{Form, Multipart};
use poem_openapi::{payload::Html, OpenApi};
use serde::Deserialize;
use tracing::{event, Level};

use crate::{disassemble, Instruction};

#[derive(Debug, Template)]
#[template(path = "main.html")]
struct MainPage;

#[derive(Debug, Deserialize)]
pub struct TableParams {
    bytes: String,
}

#[derive(Debug, Template)]
#[template(path = "table.html")]
struct TableTemplate {
    lines: Vec<Instruction>,
}

#[derive(Debug, Template)]
#[template(path = "table-error.html")]
struct TableErrorTemplate {
    illegals: Vec<(usize, String)>,
}

#[derive(Debug)]
pub struct Frontend;

#[OpenApi]
impl Frontend {
    #[oai(path = "/", method = "get")]
    pub async fn front_page(&self) -> Html<String> {
        event!(Level::INFO, "Front page");
        Html(MainPage.render().unwrap())
    }

    #[oai(path = "/table", method = "post")]
    pub async fn table(&self, Form(params): Form<TableParams>) -> Html<String> {
        event!(Level::INFO, "Table");

        let mut illegals = vec![];

        let filtered: Vec<char> = params
            .bytes
            .chars()
            .filter(|c| !c.is_ascii_whitespace()) // Strip whitespace
            .collect();

        let bytes: Vec<u8> = filtered
            // This could be manually folded or chunked with itertools,
            // but I like the simplicity of collecting and using std rust tools.
            .chunks(2)
            .enumerate()
            .filter_map(|(index, chars)| {
                let token = String::from_iter(chars);
                if let Ok(digit) = u8::from_str_radix(token.as_str(), 16) {
                    Some(digit)
                } else {
                    illegals.push((index, token));
                    None
                }
            })
            .collect();

        if illegals.is_empty() {
            let lines = disassemble(&bytes);
            Html(TableTemplate { lines }.render().unwrap())
        } else {
            Html(TableErrorTemplate { illegals }.render().unwrap())
        }
    }

    #[oai(path = "/decode", method = "post")]
    pub async fn decode_file(&self, mut multipart: Multipart) -> Html<String> {
        event!(Level::INFO, "Decode file");

        let Ok(Some(file)) = multipart.next_field().await else {
            // No file is treated as an empty file
            return Html(String::new());
        };

        Html(
            file.bytes()
                .await
                .unwrap_or_default() // Failed to parse file, treat as an empty file
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
        let client = reqwest::Client::new();

        let bytes = std::fs::read("test-bin/test1.bin").unwrap();

        let lines: String = client
            .post("http://localhost:9999/decode")
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
