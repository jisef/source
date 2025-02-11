use std::path::PathBuf;
use std::process;
use clap::{Arg, ArgMatches, Command, Parser};
pub fn get_args() -> ArgMatches {
    Command::new("Show CSV")
        .version("1.0")
        .author("jisef of GH")
        .about("Shows a CSV file")
        .arg(
            Arg::new("filename")
                .help("The file to process")
                .required(true)
                .index(1)
                .value_parser(clap::value_parser!(PathBuf))
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
        Arg::new("index")
            .long("index")
            .num_args(0)
            .help("Prints indexes to the table")
    ).arg(
        Arg::new("sql")
            .help("Run SQL on Table")
            .long("filter")
            .short('f')
            .num_args(1)
    ).arg(
        Arg::new("insert")
            .help("Insert values into csv")
            .long("insert")
            .short('i')
            .num_args(1)
    ).arg(
            Arg::new("filepath export")
                .help("Writes the Created CSV into a new File. If nothing is specified the input will be overwritten.")
                .long("export")
                .short('e')
                .num_args(0..=1)
        )
        .get_matches()
}

pub fn get_filepath() -> String {
    match get_args().get_one::<PathBuf>("filename") {
        Some(value) => value.to_str().unwrap().to_string(),
        None => String::from("export.csv") ,
    }
}


pub fn get_index() -> bool {
    get_args().contains_id("index")
}