use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use actix_cors::Cors;
use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct MutateRequest {
    #[serde(rename = "clientId")]
    client_id: usize,
    #[serde(rename = "targetText")]
    target_text: Vec<String>,
    #[serde(rename = "mutatedLength")]
    mutated_length: Option<usize>,
}

#[derive(Serialize, Debug)]
struct MutateResponse {
    result: MutateResult,
}

#[derive(Serialize, Debug)]
struct MutateResult {
    #[serde(rename = "rawContents")]
    raw_contents: Vec<String>,
    #[serde(rename = "mutatedText")]
    mutated_text: Vec<String>,
    #[serde(rename = "mutatedLength")]
    mutated_length: Option<usize>,
}

#[post("/mutate4")]
async fn mutate_text_4(req: web::Json<MutateRequest>, client: web::Data<Client>) -> impl Responder {
    let prompts = [
        "入力テキストの感想・感情・意見を真逆の意味合いに書き換えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        "入力テキストの感想・感情・意見など主観的な部分を楽観的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        "入力テキストの感想・感情・意見など主観的な部分を悲観的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        "入力テキストの感想・感情・意見など主観的な部分を自己拡張的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
    ];

    let window_id = req.client_id;
    let prompt = &prompts[window_id];
    let raw_contents = req.target_text.clone();
    let mut mutated_texts = Vec::new();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api_url = "https://api.openai.com/v1/chat/completions";

    for raw_content in &req.target_text {
        if !raw_content.trim().is_empty() {
            let content = format!(
                "{} ただし、改行は入力文そのままにすること。\n ================ \n{}",
                prompt, raw_content
            );

            let response = client
                .post(api_url)
                .bearer_auth(&api_key)
                .json(&serde_json::json!({
                    "model": "gpt-4-turbo",
                    "messages": [{"role": "user", "content": content}]
                }))
                .send()
                .await;

            match response {
                Ok(res) => {
                    let res_json = res.json::<serde_json::Value>().await.unwrap();
                    if let Some(mutated_text) =
                        res_json["choices"][0]["message"]["content"].as_str()
                    {
                        let processed_text = process_string(mutated_text.to_string());
                        mutated_texts.push(processed_text);
                    } else {
                        mutated_texts.push("Failed to mutate text.".to_string());
                    }
                },
                Err(_) => {
                    mutated_texts.push("Error communicating with API.".to_string());
                },
            }
        }
    }

    let result = MutateResponse {
        result: MutateResult {
            raw_contents: raw_contents.clone(),
            mutated_text: mutated_texts,
            mutated_length: req.mutated_length,
        },
    };

    append_to_log(&result);

    HttpResponse::Ok().json(result)
}

fn process_string(input: String) -> String {
    if let Some(pos) = input.rfind("===") {
        input[(pos + 3)..].trim().to_string()
    } else {
        input.trim().to_string()
    }
}

fn append_to_log(response: &MutateResponse) {
    let log_file = "api_log.json";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .unwrap();

    let log_entry = serde_json::to_string(response).unwrap();
    writeln!(file, "{}", log_entry).unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let client = Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(mutate_text_4)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
