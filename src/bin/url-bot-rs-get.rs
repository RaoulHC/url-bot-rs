const USAGE: &str = "
URL munching IRC bot, web page title fetching tool.

Usage:
    url-bot-rs [options] [-v...] [<url>]

Options:
    -h --help                     Show this help message.
    --version                     Print version.
    -v --verbose                  Show extra information.
    -q --quiet                    Quiet.
    -D --debug                    Print debugging information.
    -u=<val> --user-agent=<val>   Specify user-agent.
    -l=<val> --accept-lang=<val>  Specify accept-lang.
    --metadata=<val>              Enable metadata [default: true].
    --mime=<val>                  Enable mime reporting [default: true].
";

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate htmlescape;
extern crate itertools;
extern crate regex;
#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate image;
extern crate mime;
extern crate humansize;
extern crate irc;
extern crate directories;
extern crate toml;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate stderrlog;

mod buildinfo {
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
mod http {
    include!("../http.rs");
}
mod config {
    include!("../config.rs");
}

use http::resolve_url;
use config::Rtd;
use docopt::Docopt;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default)]
pub struct Args {
    flag_verbose: usize,
    flag_quiet: bool,
    flag_debug: bool,
    arg_url: String,
    flag_user_agent: Option<String>,
    flag_accept_lang: Option<String>,
    flag_metadata: bool,
    flag_mime: bool,
    /// unused
    flag_db: Option<PathBuf>,
    /// unused
    flag_conf: Option<PathBuf>,
}

fn version() -> &'static str {
    lazy_static!(
        static ref VERSION: String = format!(
            "v{}{} (build: {})",
            buildinfo::PKG_VERSION,
            buildinfo::GIT_VERSION.map_or_else(
                || String::from(""),
                |v| format!(" (git {})", v)),
            buildinfo::PROFILE
        );
    );
    &VERSION
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(version().to_owned()))
        .deserialize())
        .unwrap_or_else(|e| e.exit());

    stderrlog::new()
        .module(module_path!())
        .quiet(args.flag_quiet)
        .timestamp(stderrlog::Timestamp::Second)
        .verbosity(args.flag_verbose)
        .init()
        .unwrap();

    let mut rtd: Rtd = Rtd::default();

    if let Some(u) = args.flag_user_agent {
        debug!("setting user agent to {}", u);
        rtd.conf.params.user_agent = u;
    }
    if let Some(l) = args.flag_accept_lang {
        debug!("setting accept-lang to {}", l);
        rtd.conf.params.accept_lang = l;
    }

    rtd.conf.features.report_metadata = args.flag_metadata;
    rtd.conf.features.report_mime = args.flag_mime;

    match resolve_url(&args.arg_url, &rtd) {
        Ok(r) => info!("{}", r),
        Err(e) => error!("{}", e),
    }
}
