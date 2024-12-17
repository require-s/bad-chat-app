use crate::errors::*;
use axum::{
    extract::{Path, State}, http::header, response::IntoResponse, routing::{get, post}, Form, Router
};
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use std::{fs::read_to_string, io, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

mod errors;

pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                script src="/static/htmx.js" {}
                link rel="stylesheet" href="/static/styles.css";
            }
            body {
                main {
                    (body)
                }
            }
        }
    }
}

async fn index() -> Markup {
    page(
        "Home",
        html! {
            h1 { "A Very Bad Chat App" }
            div.messages hx-get="/messages" hx-trigger="load, every 2s" {}

            button hx-get="/messages" hx-target=".messages" { "refresh" }

            form hx-post="/messages" hx-swap="none" {
                label for="author" { "Name" }
                input name="author" type="text" placeholder="Type your name here";
                br;
                label for="content" { "Message" }
                input name="content" type="text" placeholder="Type your message here";
                br;
                button type="submit" { "Send" }
            }
        },
    )
}

async fn messages(State(state): State<MutState>) -> Markup {
    let state = state.read().await;
    html! {
        @for msg in state.messages.clone() {
            div.message {
                p.author { (msg.author) ":" }
                p.content { (msg.content) }
            }
        }
    }
}

#[derive(Deserialize, Clone)]
struct Message {
    author: Box<str>,
    content: Box<str>,
}

async fn post_message(State(state): State<MutState>, Form(msg): Form<Message>) {
    let mut state = state.write().await;
    state.messages.push(msg);
    if state.messages.len() > state.max_messages {
        state.messages.remove(0);
    }
}

async fn static_route(Path(file): Path<String>) -> Result<impl IntoResponse> {
    let ext = file.rsplit(".").next();
    let file = read_to_string("static/".to_owned() + &file)?;
    let mime = match ext {
        Some("css") => "text/css",
        Some("js") => "text/javascript",
        Some("json") => "application/json",
        Some(_) => "text/plain",
        None => "text/plain",
    };
    Ok(([(header::CONTENT_TYPE, mime)], file))
}

type MutState = Arc<RwLock<AppState>>;

#[derive(Clone)]
struct AppState {
    messages: Vec<Message>,
    max_messages: usize,
}

impl AppState {
    fn new() -> Self {
        AppState {
            messages: Vec::new(),
            max_messages: 10,
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/messages", get(messages))
        .route("/messages", post(post_message))
        .route("/static/:file", get(static_route))
        .with_state(Arc::new(RwLock::new(AppState::new())));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
