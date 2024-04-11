//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use std::time::Duration;

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::get,
    Router,
};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use clap::Parser;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli_arguments;
mod lnurl;
mod phoenixd_client;

#[tokio::main]
async fn main() {
    let cli = cli_arguments::Cli::parse();
    let verbose_level = if cli.debug { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!("lnaddress_phoenixd={verbose_level},tower_http=info,axum::rejection=trace").into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    let app = Router::new()
        .route("/.well-known/lnurlp/:username", get(lnurl::handle_lnurlp))
        .route("/.well-known/lnurlp/:username/callback", get(lnurl::lnurl_callback_handler))
        .layer(
            // `TraceLayer` is provided by tower-http
            //
            // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
            //
            // If you want to customize the behavior using closures here is how.
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                    tracing::debug!("{} {}", request.method(), request.uri().path())
                })
                .on_response(|_response: &Response, latency: Duration, _span: &Span| {
                    // ...
                    tracing::debug!("response generated in {:?}", latency)
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        );
    let app = app.fallback(handler_404);

    // run it
    let listen_address = cli.listen_host.unwrap();
    let listen_port = cli.listen_port.unwrap();
    let listener = tokio::net::TcpListener::bind(format!("{listen_address}:{listen_port}")).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404")
}