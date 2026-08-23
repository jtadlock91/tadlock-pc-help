//! Tadlock PC Help — v0: the simplest server that serves the homepage.
//!
//! What this version deliberately does NOT have yet: templating, routing
//! beyond "/", logging, graceful shutdown, static asset serving. Each of
//! those is a future step with its own lesson. v0's only job is:
//! bind a port, answer GET / with HTML.

use axum::{response::Html, routing::get, Router};

// #[tokio::main] is a macro that rewrites this function. Rust's `async fn`
// produces a state machine (a Future) that does nothing until something
// polls it. The macro expands to roughly:
//
//   fn main() {
//       tokio::runtime::Builder::new_multi_thread()
//           .enable_all()
//           .build()
//           .unwrap()
//           .block_on(async { /* your code */ })
//   }
//
// i.e. it builds the tokio runtime and hands it your async code to drive.
// There is no magic — you could write that expansion by hand.
#[tokio::main]
async fn main() {
    // A Router maps (method, path) pairs to handlers. `get(home)` says:
    // for GET requests to this path, call `home`. Any plain async function
    // whose return type implements `IntoResponse` can be a handler — no
    // trait implementations or registration boilerplate on your side.
    let app = Router::new().route("/", get(home));

    // 0.0.0.0 = listen on all interfaces, which is what you want inside a
    // Docker container (127.0.0.1 would only be reachable from inside the
    // container itself). Port 8080 because the container will run as a
    // non-root user and ports <1024 are privileged.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind 0.0.0.0:8080 — is something already on that port?");

    println!("listening on http://{}", listener.local_addr().unwrap());

    // axum::serve runs the accept loop: accept connection → parse HTTP
    // (hyper does this) → route → call handler → write response. It only
    // returns on error, hence the trailing .expect.
    axum::serve(listener, app)
        .await
        .expect("server error");
}

// The handler. `Html<T>` is a wrapper that sets `Content-Type: text/html`
// on the response — without it, the browser would receive text/plain and
// show you the raw markup.
//
// `include_str!` embeds the file's contents INTO THE BINARY at compile
// time. That means: single self-contained executable, no filesystem reads
// at runtime, nothing to forget when copying into a Docker image. The
// tradeoff: changing the HTML requires a rebuild. For v0 that's fine —
// and it makes the deployment story maximally simple. When the HTML gets
// real, we'll revisit (templating with askama keeps this same
// compile-time property; ServeDir would trade it for runtime flexibility).
async fn home() -> Html<&'static str> {
    Html(include_str!("../assets/index.html"))
}
