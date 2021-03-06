#[macro_use]
extern crate lazy_static;

mod client;
mod error;
mod request;

use clap::{App, Arg, ArgGroup};

use crate::client::Client;

fn main() {
    let matches = App::new("rural")
        .version(env!("CARGO_PKG_VERSION"))
        .author("https://github.com/saghm/rural")
        .about("Command-line HTTP client")
        .arg(
            Arg::with_name("METHOD")
                .help("HTTP request method to use")
                .required(true)
                .index(1)
                .possible_values(&["delete", "get", "head", "options", "patch", "post", "put"]),
        )
        .arg(
            Arg::with_name("URL")
                .help("URL to request")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("PARAM")
                .help(
                    "querystring parameter (i.e `key==value`), body parameter (i.e `key=value)`, \
                     json parameter (i.e. `key:=value`), or header (`name:value`)",
                )
                .index(3)
                .multiple(true),
        )
        .arg(
            Arg::with_name("headers")
                .help("Print response headers instead of body")
                .short("d")
                .long("headers"),
        )
        .arg(
            Arg::with_name("both")
                .help("Print both response headers and body")
                .conflicts_with("headers")
                .short("b")
                .long("both"),
        )
        .arg(
            Arg::with_name("suppress-info")
                .help("Do not print the HTTP version and response status code")
                .short("s")
                .long("suppress-info")
                .requires("headers-printed"),
        )
        .group(ArgGroup::with_name("headers-printed").args(&["headers", "both"]))
        .arg(
            Arg::with_name("no-color")
                .help("Do not colorize the output")
                .short("n")
                .long("no-color"),
        )
        .arg(
            Arg::with_name("form")
                .help("Send POST data as a form rather than JSON")
                .short("f")
                .long("form"),
        )
        .arg(
            Arg::with_name("out")
                .help("Download output to specified file")
                .short("-o")
                .long("--out")
                .takes_value(true)
                .value_name("OUT"),
        )
        .get_matches();

    let client = Client::new(matches);

    match client.execute() {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("{}", err),
    }
}
