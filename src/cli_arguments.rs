use clap::Parser;

use crate::phoenixd_client;
use crate::lnurl;

// Simple program for resolving lnaddress with phoenixd
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// IPv4 or IPv6 Hostname for the lnurl server shall listen on
    #[arg(short = 'l', long = "listen", env = "HOST", default_value="127.0.0.1", value_name = "IP")]
    pub listen_host: Option<String>,

    /// Port number for the lnurl server shall listen on
    #[arg(short = 'p',long = "port", env = "PORT", default_value="3000", value_name = "PORT")]
    pub listen_port: Option<String>,

    /// Domain name which the server responds to, by default this is auto resolved
    #[arg(long = "domain", env = "DOMAIN", value_name = "Domain")]
    pub domain_name: Option<String>,

    /// Usernames seperated with space which shall accept payments
    #[arg(short, long, env = "USERNAMES", default_value="*", value_name = "USERNAMES",num_args(0..))]
    pub accepted_username: Option<Vec<String>>,

    //cli arguments for Phoenixd
    #[clap(flatten)]
    pub phoenixd_opts: phoenixd_client::PhoenixdSubOpts,

    //cli arguments for lnurl
    #[clap(flatten)]
    pub lnurl_opts: lnurl::LnurlSubOpts,

    /// Turn debugging information on
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,
}
