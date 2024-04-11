use std::collections::HashMap;

use askama_axum::IntoResponse;
use axum::extract::{Host, Path, Query};
use axum::http::{header, HeaderMap, HeaderValue};
use clap::Parser;

use crate::{cli_arguments, phoenixd_client};

#[derive(Parser, Debug)]
pub struct LnurlSubOpts {
    /// Sets an identifying name  which is displayed when paying
    #[arg(long, env, value_name = "STRING", default_value = "Satoshi")]
    pub lnurl_payment_identify: String,

    /// Sets a message which is displayed when paying
    #[arg(long, env, value_name = "STRING", default_value = "Hello World")]
    pub lnurl_payment_description: String,

    /// Allow payee to add a comment together with the payment
    #[arg(long, env, value_name = "lenght", default_value_t = 0)]
    pub lnurl_allow_note: u8,

    /// leave a greating when the payment is done
    #[arg(long, env, value_name = "STRING", default_value = "")]
    pub lnurl_greeting: Option<String>,

    /// minimum ammount in milisatoshi to send
    #[arg(long, env, value_name = "lenght", default_value_t = 1000)]
    pub lnurl_minimum_sendable_milisats: u64,

    /// maximym ammount in milisatoshi to send
    #[arg(long, env, value_name = "lenght", default_value_t = 2100000000)]
    pub lnurl_maximum_sendable_milisats: u64,
}

#[derive(askama::Template)]
#[template(path = "lnaddress.html")]
struct LnurlpTemplate<'a> {
    status: &'a str,
    name: String,
    min_sendable: u64,
    max_sendable: u64,
    description: String,
    comment_length: u8,
    callback_host: String,
    callback_proto: String,
    error_message: &'a str,
}

#[derive(askama::Template)]
#[template(path = "lnurlp.html")]
struct LnurlpCallbackTemplate<'a> {
    status: &'a str,
    ln_data: String,
    success_message: String,
    error_message: &'a str,
}

pub async fn lnurl_callback_handler(
    Path(username): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // LUD-06: payRequest base spec.
    // LUD-09: successAction field for payRequest.
    // LUD-12: Comments in payRequest.

    let cli = cli_arguments::Cli::parse();

    //Data reseived from Wallet
    // <callback><?|&>
    // amount=<milliSatoshi>
    // &nonce=<hex(8 bytes of random data)>
    // &fromnodes=<nodeId1,nodeId2,...>
    // &comment=<String>
    // &proofofpayer=<hex(ephemeral secp256k1 public key bytes)>

    let amount_sat = params.get("amount").unwrap().parse::<u64>().unwrap() / 1000;
    let description = params.get("comment");
    //Make request to phoenixd with parameters retreived from callback GET
    // Phoenix POST /createinvoice
    // DATA: description, amount_sat

    let invoice =
        phoenixd_client::create_invoice(amount_sat, description.unwrap_or(&String::new()), "rust")
            .await;
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    let configured_usernames = cli.accepted_username.unwrap();
    return if configured_usernames.contains(&username)
        || configured_usernames.contains(&"*".to_string())
    {
        (
            headers,
            LnurlpCallbackTemplate {
                status: "ok",
                ln_data: invoice.serialized,
                success_message: cli.lnurl_opts.lnurl_greeting.unwrap(),
                error_message: "",
            },
        )
    } else {
        (
            headers,
            LnurlpCallbackTemplate {
                status: "error",
                ln_data: "".to_string(),
                success_message: "".to_string(),
                error_message: "User not found",
            },
        )
    };
}

pub async fn handle_lnurlp(
    Path(username): Path<String>,
    Host(auto_callback_host): Host,
    req_headers: HeaderMap,
) -> impl IntoResponse {
    // LUD-06: payRequest base spec.
    // LUD-11: Disposable and storeable payRequests.


    let mut headers = HeaderMap::new();
    let cli = cli_arguments::Cli::parse();

    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    let configured_usernames = cli.accepted_username.unwrap();

    let callback_host = if cli.domain_name.is_some() { cli.domain_name.unwrap() } else { auto_callback_host };
    let callback_proto = req_headers.get("X-Forwarded-Proto")
        .unwrap_or(&HeaderValue::from_static("http")).to_str().unwrap().to_string();

    return if configured_usernames.contains(&username)
        || configured_usernames.contains(&"*".to_string())
    {
        (
            headers,
            LnurlpTemplate {
                status: "ok",
                min_sendable: cli.lnurl_opts.lnurl_minimum_sendable_milisats,
                max_sendable: cli.lnurl_opts.lnurl_maximum_sendable_milisats,
                name: cli.lnurl_opts.lnurl_payment_identify.to_string(),
                description: cli.lnurl_opts.lnurl_payment_description.to_string(),
                comment_length: cli.lnurl_opts.lnurl_allow_note,
                callback_host,
                callback_proto,
                error_message: "",
            },
        )
    } else {
        (   // Data for error
            headers,
            LnurlpTemplate {
                status: "error",
                min_sendable: 0,
                max_sendable: 0,
                name: "".to_string(),
                description: "".to_string(),
                comment_length: 0,
                callback_host: "".to_string(),
                callback_proto: "".to_string(),
                error_message: "User not found",
            },
        )
    };
}
