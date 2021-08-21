use clap::{App, AppSettings, Arg, ArgMatches};

pub fn run() {
    let matches = App::new("keywords")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::PropagateVersion)
        .subcommand(
            App::new("search")
                .arg(
                    Arg::new("languages")
                        .short('l')
                        .long("languages")
                        .required(false)
                        .takes_value(true),
                )
                .arg(Arg::new("keyword").required(true).takes_value(true)),
        )
        .get_matches();

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

            let keyword = matches.value_of("keyword").unwrap();

            match data::api::keyword::search(keyword, languages) {
                Ok(_) => println!("MATCH"),
                Err(err) => println!("ERROR: {}", err),
            }
        }
        _ => unreachable!(),
    }
}
