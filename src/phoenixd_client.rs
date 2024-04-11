use std::collections::HashMap;
use std::{env, fs};

use crate::cli_arguments;
use clap::Parser;
use reqwest::Client;
use serde::Deserialize;

#[derive(Parser, Debug)]
pub struct PhoenixdSubOpts {
    /// Sets a custom path for phoenixd config file, default is $HOME/.phoenix/phoenix.conf
    #[arg(long, env, value_name = "FILE")]
    pub phoenixd_config: Option<String>,

    /// Sets a hostname to phoenixd
    #[arg(long, env, value_name = "URL", default_value = "http://127.0.0.1")]
    phoenixd_url: Option<String>,

    /// Sets a port to phoenixd
    #[arg(long, env, value_name = "PORT", default_value = "9740")]
    phoenixd_port: Option<String>,

    /// Configure an username to access phoenixd API
    #[arg(long, env, value_name = "username")]
    phoenixd_username: Option<String>,

    /// Configure a password to access phoenixd API, default is 9740
    #[arg(long, env, value_name = "password")]
    phoenixd_password: Option<String>,
}

pub struct AuthCredentials {
    pub username: String,
    pub password: String,
}

pub fn get_auth_config() -> AuthCredentials {
    let cli = cli_arguments::Cli::parse();
    return if cli.phoenixd_opts.phoenixd_password.is_some() {
        // Use credentials from provided from cli arguments or environment variables
        AuthCredentials {
            username: cli
                .phoenixd_opts
                .phoenixd_username
                .unwrap_or("http-password".to_string()),
            password: cli.phoenixd_opts.phoenixd_password.unwrap(),
        }
    } else {
        // Use credentials from Phoenix server default folder, if path is not provided
        let binding = env::var_os("HOME").unwrap();
        let home_dir = binding.to_str().unwrap();
        let config_path;

        if cli.phoenixd_opts.phoenixd_config.is_none() {
            config_path = format!("{home_dir}/.phoenix/phoenix.conf");
        } else {
            config_path = cli.phoenixd_opts.phoenixd_config.unwrap();
        }

        let file_content: String =
            fs::read_to_string(config_path).expect("Phoenix config file not found");
        let mut data = HashMap::new();

        for line in file_content.trim().split('\n') {
            // Split each line by '=' character
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                // Insert key-value pairs into the HashMap
                data.insert(parts[0].trim(), parts[1].trim());
            }
        }

        AuthCredentials {
            username: "http-password".to_string(),
            password: data.get("http-password").unwrap().to_string(),
        }
    };
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct PhoenixInvoice {
    pub amountSat: u64,
    pub paymentHash: String,
    pub serialized: String,
}

pub async fn create_invoice(
    amount_sat: u64,
    description: &str,
    external_id: &str,
) -> PhoenixInvoice {
    //let local_cli = SubOpts::parse();
    let cli = cli_arguments::Cli::parse();
    let client = Client::new();
    let parameters = [
        ("description", description),
        ("amountSat", &amount_sat.to_string()),
        ("externalId", external_id),
    ];
    let phoenix_server_url = cli.phoenixd_opts.phoenixd_url.unwrap();
    let phoenix_server_port = cli.phoenixd_opts.phoenixd_port.unwrap();
    let response = client
        .post(format!(
            "{phoenix_server_url}:{phoenix_server_port}/createinvoice"
        ))
        .basic_auth(get_auth_config().username, Some(get_auth_config().password))
        .form(&parameters)
        .send()
        .await
        .unwrap();

    response.json::<PhoenixInvoice>().await.unwrap()
}
