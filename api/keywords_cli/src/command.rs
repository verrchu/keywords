use std::process;

use clap::{App, AppSettings, Arg, ArgMatches};

pub fn run() -> () {
    let matches = App::new("keywords")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .subcommand(
            App::new("list")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(App::new("languages"))
                .subcommand(App::new("versions").arg(Arg::new("language").required(true))),
        )
        .get_matches();

    process(matches);
}

fn process(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("list", matches)) => match matches.subcommand() {
            Some(("languages", _)) => {
                let languages = data::api::languages::list();
                todo!();
            }
            Some(("versions", matches)) => {
                let language = get_arg(matches, "language");
                todo!();
            }
            other => handle_invalid_subcommand(other),
        },
        other => handle_invalid_subcommand(other),
    }
}

fn get_arg(matches: &ArgMatches, name: &str) -> String {
    if let Some(value) = matches.value_of(name) {
        value.to_owned()
    } else {
        log::error!("expected argument {}", name);

        process::exit(1);
    }
}

fn handle_invalid_subcommand(subcommand: Option<(&str, &ArgMatches)>) {
    match subcommand {
        Some((command, _)) => log::error!("unknown subcommand: {}", command),
        None => log::error!("subcommand expected"),
    }

    process::exit(1);
}
