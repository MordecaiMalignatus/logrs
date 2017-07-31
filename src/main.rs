extern crate clap;
extern crate chrono;
extern crate grep;
extern crate ignore;
extern crate regex;
extern crate termcolor;
extern crate time;
extern crate ulid;

use clap::{Arg, App, AppSettings, SubCommand};
use regex::Regex;

mod config;
mod logger;
mod search;
mod show;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let matches = App::new("logrs")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .subcommand(SubCommand::with_name("add")
            .about("Add a record")
            .arg(Arg::with_name("TEXT")
                .required(true)
                .help("Text you want to record, - for stdin")
            )
        )
        .subcommand(SubCommand::with_name("show")
            .about("Display specific record or date")
            .arg(Arg::with_name("WHAT")
                .required(true)
                .help("Values can be: today, yesterday, specific date (2017-06-05) or ID of record (01BP42DBMPY33X8ZB2GW7CVM2Q)")
            )
        )
        .subcommand(SubCommand::with_name("search")
            .about("Search all records")
            .arg(Arg::with_name("pattern")
                .required(true)
                .help("Regex pattern")
            )
        )
        .get_matches();

    // Try to load config, quit on error.
    let config = match config::default_config() {
        Ok(conf) => {conf},
        Err(e) => {
            eprintln!("{}", e);
            return
        },
    };
    
    // Match subcommands to their functions.
    // TODO: refactor modules to either return Result and handle printing errors here,
    // or print all errors directly from modules (preferably the first option).
    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            match logger::log(add_matches.value_of("TEXT").unwrap(), &config) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{}", e);
                    return
                }
            }
        },
        ("show", Some(show_matches)) => {
            match show_matches.value_of("WHAT").unwrap() {
                // TODO: try changing commands "today" and "yesterday" into subcommands.
                "today" => show::show_today(&config),
                "yesterday" => show::show_yesterday(&config),
                date_or_id => {
                    // Match date with regex, and dispatch proper show function.
                    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
                    if re.is_match(date_or_id) {
                        show::show_date(date_or_id, &config);
                    } else {
                        show::show_id(date_or_id, &config)
                    } 
                },
            }
        },
        ("search", Some(search_matches)) => {
            let pattern = search_matches.value_of("pattern").unwrap();
            search::custom_grep(pattern, &config);
        },
        _ => unreachable!(),
    }
}