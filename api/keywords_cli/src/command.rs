use std::process;

use clap::{App, AppSettings, Arg, ArgMatches};

pub fn run() {
    let matches = App::new("keywords")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .subcommand(
            App::new("search")
                .arg(
                    Arg::new("languages")
                        .short('l')
                        .long("languages")
                        .required(false)
                        .takes_value(true),
                )
                .arg(Arg::new("word").required(true).takes_value(true)),
        )
        .get_matches();

    dbg!(&matches);

    process(matches);
}

fn process(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("search", matches)) => {
            log::debug!("executing search subcommand");

            let languages: Option<Vec<String>> = matches
                .value_of("languages")
                .map(|languages| languages.split(',').map(ToOwned::to_owned).collect());

            log::debug!("languages supplied: {:?}", languages);

            let word = matches.value_of("word").unwrap();

            let occurences = data::api::keyword::search(&word, languages);

            dbg!(occurences);
        }
        _ => unreachable!(),
    }
}
