rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result as ActixResult};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Value};
use std::{vec::Vec, fs};

#[derive(Deserialize)]
struct ExecuteRequest {
    command: String,
}

#[derive(Serialize)]
struct ExecuteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn fetch_data(entity: &str, _timeframe: Option<&str>, source: Option<&str>) -> Result<Vec<Value>, String> {
    if source.is_none() {
        if entity == "appointments" {
            let file_path = "sources/appointments.json";
            let data = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(_) => return Err("Failed to read data from default source".to_string()),
            };

            let parsed_data: Vec<Value> = match serde_json::from_str(&data) {
                Ok(parsed) => parsed,
                Err(_) => return Err("Failed to parse JSON data from default source".to_string()),
            };

            return Ok(parsed_data);
        } else {
            return Err(format!("Data not found for entity: {}", entity));
        }
    } else {
        //Specific Source
        return Err("Specific source not yet implemented".to_string());
    }
    

    // Placeholder data source: For now, we only support "appointments" entity.
    
    
}

fn execute_charcot_command(command: &str) -> Result<String, String> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.len() < 2 || parts[0] != "fetch" {
        return Err("Invalid command".to_string());
    }

    let entity = parts[1];
    let mut timeframe = None;
    let mut source = None;

    // Basic parsing for timeframe and source
    let mut i = 2;
    while i < parts.len(){
        if parts[i] == "from"{
            if i + 1 < parts.len() {
                source = Some(parts[i + 1]);
                i += 2;
            }else {
                return Err("Invalid 'from' clause".to_string());
            }
        } else {
             timeframe = Some(parts[i]);
            i += 1;
        }
    }

    match fetch_data(entity, timeframe, source) {
        Ok(data) => {
            // Basic filtering based on timeframe
            if let Some(tf) = timeframe{
                if tf != "day"{
                    return Err("Timeframe filtering not implemented".to_string())
                }
            }
            serde_json::to_string(&data).map_err(|e| format!("JSON serialization error: {}", e))
        }
        Err(e) => Err(e),
    }
    
}

async fn execute(req: web::Json<ExecuteRequest>) -> impl Responder {
    let command = &req.command;
    match execute_charcot_command(command) {
        Ok(output) => HttpResponse::Ok().json(ExecuteResponse {
            output: Some(output),
            error: None,
        }),
        Err(error) => HttpResponse::BadRequest().json(ExecuteResponse {
            output: None,
            error: Some(error),
        }),
    }
}

async fn index() -> impl Responder {
    let html = include_str!("../static/index.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/api/execute").route(web::post().to(execute)))
            .service(actix_web_static_files::ResourceFiles::new(
                "/",
                &static_files::FS,
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(rust_embed::RustEmbed)]
#[folder = "static/"]
struct static_files;