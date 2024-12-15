use axum::{routing::get, Router};
use maud::{html, Markup};
use std::io;
use tokio::net::TcpListener;

// Create a simple route handler function
async fn hello_world() -> Markup {
    html! {
        h1 { "yoo" }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Router::new().route("/", get(hello_world));

    // Start the server
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;


    Ok(())
}
