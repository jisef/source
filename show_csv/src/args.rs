use clap::{Arg, ArgMatches, Command};

pub fn get_args() -> ArgMatches {
    let matches = Command::new("Show CSV")
        .version("1.0")
        .author("jisef of GH")
        .about("Shows a CSV file")
        .arg(
            Arg::new("filename")
                .help("The file to process")
                .required(true)
                .index(1),
        ).arg(
        Arg::new("seperator")
            .short('s')
            .long("seperator")
            .help("Specifies seperating value")
            .num_args(1)
            .value_parser(clap::builder::ValueParser::new(|s: &str| {
                if s.len() == 1 {
                    Ok(s.chars().next().unwrap()) // Accept only single-character input
                } else {
                    Err(format!("Expected a single character, got '{}'", s))
                }
            }))
            .default_value(","),
    ).arg(
        Arg::new("sql")
            .help("Run SQL on Table")
            .long("filter")
            .short('f')
            .num_args(0..1)
    ).get_matches();

    return matches;
}