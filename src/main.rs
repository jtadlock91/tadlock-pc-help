//! Tadlock PC Help — services now modeled as data, rendered via askama.

use askama::Template;
use axum::{http::StatusCode, response::Html, routing::get, Router};

struct Service {
    name: &'static str,
    price: &'static str,
}

// The derive is where the work happens: at COMPILE time, askama reads
// templates/index.html, parses it, and generates Rust code that renders
// it. The template becomes part of the program. Reference a field this
// struct doesn't have in the template, and compilation fails — not a
// blank spot you find on the live page next week.
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    services: Vec<Service>,
    phone: &'static str,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind 0.0.0.0:8080 — is something already on that port?");

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn home() -> Result<Html<String>, StatusCode> {
    let page = IndexTemplate {
        services: vec![
            Service { name: "Custom PC Build", price: "$125" },
            Service { name: "Upgrades & Repairs", price: "$40–60" },
            Service { name: "Home Wi-Fi & Network Help", price: "$40–60" },
            Service { name: "Slow Computer Tune-Up", price: "$40" },
            Service { name: "Custom Parts List", price: "$30" },
            Service { name: "Home & Office Tech Help", price: "$40" },
            Service { name: "Mail-In Repair", price: "$40 diagnostic" },
        ],
        phone: "(509) 994-0005",
    };
    page.render()
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
