use askama::Template;
use axohtml::{html, dom::DOMTree};
use axum::{Router, routing::{get, post}, response::Html, Form};
use serde::Deserialize;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(index))
        .route("/decrypt", post(decrypt))
        .route("/encrypt", post(encrypt));
    
    axum::Server::bind(&"0.0.0.0:3000".parse()?).serve(app.into_make_service()).await.unwrap();

    Ok(())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
}

async fn index() -> Html<String> {
    Html(IndexTemplate {}.render().unwrap().to_string())
}

#[derive(Deserialize, Debug)]
struct ShiftRequest {
    text: String,
}

#[derive(Template)]
#[template(path = "shift.html",escape = "none")]
struct ShiftTemplate {
    text: String,
}

async fn decrypt(Form(data): Form<ShiftRequest>) -> Html<String> {
    let mut shifted = String::new();
    for c in data.text.chars() {
        if c.is_ascii_uppercase() {
            if c == 'A' {
                shifted.push('Z');
                continue;
            }
            let shifted_char = (c as u8 - 1) as char;
            shifted.push(shifted_char);
        } else if c.is_ascii_lowercase() {
            if c == 'a' {
                shifted.push('z');
                continue;
            }
            let shifted_char = (c as u8 - 1) as char;
            shifted.push(shifted_char);
        } else {
            shifted.push(c);
        }
    }
    tracing::info!("shift: {:?}", data);
    Html(ShiftTemplate { text: shifted }.render().unwrap().to_string())
}

async fn encrypt(Form(data): Form<ShiftRequest>) -> Html<String> {
    let mut shifted = String::new();
    for c in data.text.chars() {
        if c.is_ascii_uppercase() {
            if c == 'Z' {
                shifted.push('A');
                continue;
            }
            let shifted_char = (c as u8 + 1) as char;
            shifted.push(shifted_char);
        } else if c.is_ascii_lowercase() {
            if c == 'z' {
                shifted.push('a');
                continue;
            }
            let shifted_char = (c as u8 + 1) as char;
            shifted.push(shifted_char);
        } else {
            shifted.push(c);
        }
    }
    tracing::info!("shift: {:?}", data);
    Html(ShiftTemplate { text: shifted }.render().unwrap().to_string())
}
