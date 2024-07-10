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
        "入力テキストの感想・感情・意見など主観的な部分を理想主義的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        "入力テキストの感想・感情・意見など主観的な部分を批判的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        "入力テキストの感想・感情・意見など主観的な部分を諦念的、現実主義的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        "入力テキストの感想・感情・意見など主観的な部分を自責的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        "入力テキストの感想・感情・意見など主観的な部分を他責的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        "入力テキストの文体を論文で記述するような文体にしてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        "入力テキストの文体をポエム・詩のように感情的に、情緒的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        "以下の文章において、書き手の主観的な感想や意見が含まれている部分を特定し、それらをより客観的で事実に基づいた表現に書き換えてください。感情的な言葉や個人的な判断を避け、中立的な立場から観察可能な事実のみを述べるようにしてください。ただし、以下の点に注意してください:\
        1. 客観的な事実や情報は変更しないでください。\
        2. 文章の全体的な構造、長さ、および文体は可能な限り維持してください。\
        3. 専門用語や固有名詞はそのまま使用してください。\
        4. 数値データや統計情報は正確に保持してください。\
        5. 引用や参照がある場合は、それらを保持してください。\
        6. 文章のトーンや論理の流れを大きく変えないようにしてください。\
        7. 主観的表現を客観的に書き換える際は、可能な限り具体的なデータや事例を用いてください。\
        8. 曖昧な表現や一般化を避け、具体的かつ明確な表現を心がけてください。\
        9. 必要に応じて、情報源や根拠を追加してください。\
        10. 書き換えた後、元の文章と比較して変更した箇所を簡潔に説明し、その理由を述べてください。\
        この作業を通じて、文章の客観性と信頼性を高めることを目指してください。"
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
