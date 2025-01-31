mod print;
mod sql;
mod args;

use std::fs;
use colored::{Colorize};
use pad::{PadStr};
use clap::{Arg, Command, Subcommand};
use std::io;
use std::io::Write;

fn main() {
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
                .num_args(0..)
        ).get_matches();

    let path: &String = matches
        .get_one::<String>("filename")
        .expect("Filename is required");

    let seperator: char = *matches
        .get_one::<char>("seperator")
        .expect("Default value is + ,");
    
    let filters: Vec<String> = matches.get_many("sql").unwrap_or_default().cloned().collect();
    let bool = filters.len() == 0;
    let filters = if bool {
        println!("SQL Query: ");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        input
    } else {
        filters.iter().map(String::from).collect()
    };

    println!("{}", filters);



    

    let sigma = read_csv(String::from(path));
    let max_width_per_column = print::get_max_width_per_column(&sigma, seperator);
    let sigma = sigma.lines();
    print::print_lines_default(max_width_per_column, sigma.clone(), seperator, 2);
    print::print_table_with_comfy_table(sigma);
    let s = print::init_colors(69);



    let s: String = match sql::validate_sql(filters) {
        Ok(valid_query) => valid_query, // Store the valid SQL string
        Err(e) => panic!("SQL validation failed: {}", e),
    };

    println!("Valid SQL: {}", s);
}



fn read_csv(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).unwrap_or_else(|error| {
        println!("Es ist ein Fehler aufgetreten!!! {}", error);
        String::new()
    });
    contents
    //let contents = fs::read_to_string(file_path)
}