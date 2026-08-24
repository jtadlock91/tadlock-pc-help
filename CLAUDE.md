# CLAUDE.md — Tadlock PC Help (Rust rebuild)

## What this project is

Rebuild of pcrepair.tadlockfamily.net ("Tadlock PC Help") in Rust.

**The goal is learning Rust, not solving a performance problem.** The current
site is a single static HTML file served by nginx:alpine in Docker — already
as lightweight as possible. There is no technical need driving this rebuild,
and that framing is intentional. Prioritize John understanding what's
happening and why over producing working code fast.

## Ground rules

1. **Explain why, not just what.** What a crate/pattern does and why it's the
   idiomatic choice — not just working code.
2. **Don't over-engineer.** This is a business-card website. Real, practical
   Rust — not enterprise architecture. No feature gets added without a reason
   a personal site actually has.
3. **John runs anything needing sudo or deployment credentials himself.**
   Provide commands for him to run; never assume deploy access.
4. **Scope: this website only.** John has other concurrent Rust projects
   (ISO fork, file-upload tool, API clients) — they are out of scope here.

## Site content (source of truth)

Services and pricing:
- Custom PC Build — $125
- Upgrades & Repairs — $40–60
- Home Wi-Fi & Network Help — $40–60
- Slow Computer Tune-Up — $40
- Custom Parts List — $30
- Home & Office Tech Help — $40
- Mail-In Repair — $40 diagnostic

Contact: (509) 994-0005 (floating "text me" button on the old site)

Old site features to eventually reach parity with: service cards with
pricing, FAQ accordion, floating text-me button, About section, Open Graph
tags for Facebook link previews.

## Infrastructure

- Deployed on John's home NAS as a Docker container
- Reached through a Cloudflare Tunnel (no ports exposed; TLS terminates at
  Cloudflare — the app serves plain HTTP internally)
- Old container: nginx:alpine serving one static file
- New container: this binary listening on 0.0.0.0:8080

## Tech decisions log

- **axum 0.8 + tokio**: ecosystem-standard web stack; same tokio world as
  John's other Rust projects (reqwest). Chosen over raw std::net (teaches
  HTTP trivia, not idiomatic Rust) and over heavier frameworks
  (over-engineering).
- **Explicit tokio features** (rt-multi-thread, macros, net) instead of
  "full" — know what you depend on, faster compiles.
- **include_str! for HTML in v0**: single self-contained binary, simplest
  possible deploy. Tradeoff (rebuild to edit HTML) accepted for now.
- **Port 8080, bind 0.0.0.0**: non-privileged port, reachable from outside
  the container.

## Roadmap (one lesson per step)

- [x] v0: axum serves embedded placeholder HTML on GET /
- [ ] Logging with tracing (see requests happen)
- [ ] Graceful shutdown (SIGTERM handling so `docker stop` is clean)
- [ ] Dockerfile (multi-stage: cargo build → scratch/distroless image)
- [ ] Deploy behind the existing Cloudflare Tunnel

## Update — real content pass (Aug 2026)
- Services and FAQ ported as `Vec<Service>` / `Vec<Faq>` structs rendered via askama; rest of the page (hero, about, promise, contact, area tags, steps) stayed as static HTML in the template — no benefit to modeling one-off content as structs.
- `Faq.answer` uses `{{ item.answer|safe }}` since two answers embed a real `<a href="tel:...">` link; safe only because the content is hardcoded by the site owner, never visitor input.
- Static assets: real site has none locally (fonts/icon/analytics are CDN or inline) — the embed-vs-ServeDir decision is moot until a real local asset (e.g. a logo image) gets added.
