//! Tadlock PC Help — request logging via tracing, tuned to INFO level.

use askama::Template;
use axum::{http::StatusCode, response::Html, routing::get, Router};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

struct Service {
    name: &'static str,
    description: &'static str,
    price: &'static str,
    price_note: &'static str,
}

struct Faq {
    question: &'static str,
    answer: &'static str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    services: Vec<Service>,
    faqs: Vec<Faq>,
    phone_display: &'static str,
    phone_link: &'static str,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let app = Router::new()
        .route("/", get(home))
        .layer(
            TraceLayer::new_for_http()
                // TraceLayer's defaults are DEBUG — invisible under our
                // "info" filter. Bumping both the span (request start,
                // carries method/path) and the response event (status,
                // latency) to INFO makes them show up without needing
                // RUST_LOG=debug, which would also flood the output with
                // every dependency's internal debug noise.
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind 0.0.0.0:8080 — is something already on that port?");

    tracing::info!(addr = %listener.local_addr().unwrap(), "listening");

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn home() -> Result<Html<String>, StatusCode> {
    let page = IndexTemplate {
        services: vec![
            Service {
                name: "Custom PC Build & Setup",
                description: r#"You buy the parts (or I'll pick them with you). I handle assembly, clean cable management, BIOS updates, Windows install, and full stress testing — ready to use day one."#,
                price: "$125",
                price_note: "flat · parts not included",
            },
            Service {
                name: "Upgrades & Repairs",
                description: r#"Speed your computer up with more memory, faster storage, or a better processor or graphics card — I handle the part-matching, the installation, and the testing. Overheating fixes and "it won't turn on" diagnostics with an honest verdict. If it needs a whole new heart (processor and motherboard), that's priced as a rebuild."#,
                price: "$40–60",
                price_note: "flat · quoted up front",
            },
            Service {
                name: "Home Wi-Fi & Network Help",
                description: r#"Dead zones, slow corners of the house, router setup, and "the smart TV won't connect" calls. I'll fix what you have or set up better coverage for the whole house — and when I'm done, your phones, TVs, and printers all reconnect on their own. No walking room to room typing the Wi-Fi password into every gadget you own."#,
                price: "$40–60",
                price_note: "flat · whole-home setups quoted",
            },
            Service {
                name: "Slow Computer Tune-Up",
                description: r#"Cleanup, updates, startup fixes, and speed restoration for desktops and laptops that have lost their step."#,
                price: "$40",
                price_note: "flat",
            },
            Service {
                name: "Custom Parts List",
                description: r#"Tell me your budget and what the machine needs to do. You get a complete, no-junk parts list with links — order it all yourself, or have me build it."#,
                price: "$30",
                price_note: "credited toward a build",
            },
            Service {
                name: "Home & Office Tech Help",
                description: r#"Printer setup, new computer setup, and moving your files safely from the old machine to the new one. The "I just need someone to make it work" service."#,
                price: "$40",
                price_note: "flat",
            },
            Service {
                name: "Mail-In Repair",
                description: r#"Outside the Basin? Ship me your system and I'll diagnose it, quote the fix before any work starts, and ship it back tested. Message me first for packing instructions — graphics cards need to come out for shipping, and if you've never done that, no worries: I'll walk you through it over the phone step by step. It's easier than it sounds."#,
                price: "$40 diagnostic",
                price_note: "+ repair quote · shipping both ways paid by customer",
            },
        ],
        faqs: vec![
            Faq {
                question: "Do I need an appointment?",
                answer: r#"Nope — just text or call <a href="tel:+15099940005" style="color:var(--copper-deep);">(509) 994-0005</a> and tell me what's going on. I'll usually get back to you the same evening and we'll set up a time that works."#,
            },
            Faq {
                question: "How long does a repair take?",
                answer: r#"Most tune-ups and upgrades are back to you in one to three days. New builds usually take a few days once all the parts have arrived. I'll give you a timeframe with your quote, and I'll text you when it's done."#,
            },
            Faq {
                question: "Do you work on laptops?",
                answer: r#"Yes — tune-ups, cleanup, and memory or storage upgrades on most models (a storage upgrade is the single best way to make an older laptop feel new again). Cracked screens and battery replacements depend on the laptop, so send me the make and model and I'll tell you straight whether it's worth doing."#,
            },
            Faq {
                question: "What if my computer isn't worth fixing?",
                answer: r#"Then I'll tell you that, and the opinion costs nothing. If you do need a replacement, I can put together a parts list or build that fits your budget — but I'd rather lose a repair job than have you waste money."#,
            },
            Faq {
                question: "Will my files be safe?",
                answer: r#"Your photos, documents, and family stuff are the most valuable part of the machine, and I treat them that way. Before any work that could touch your files, the first thing we'll do together is back everything up — I'll handle it and show you what's saved where, so nothing is at risk no matter what the repair turns up. And I don't go poking through your stuff, period."#,
            },
        ],
        phone_display: "(509) 994-0005",
        phone_link: "+15099940005",
    };
    page.render()
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
