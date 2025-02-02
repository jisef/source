mod print;
mod sql;
mod args;
mod csv;

use std::fs;
use colored::{Colorize};
use pad::{PadStr};
use clap::{Arg, Command, Subcommand};
use std::io;
use std::io::Write;
use sqlparser::ast::{Select, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use crate::csv::create_csv_object;
use crate::sql::run_sql_on_csv;
use sqlparser::ast::{ Expr, SelectItem};
use sqlparser::test_utils::table;

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
        println!("SQL Query: (GenericDialect)");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.to_string();
        input
    } else {
        //filters.iter().map(String::from).collect();
        let mut query: String = String::new(); // garuanted its not null
        for i in 0..filters.len() {
            query.push_str(filters[i].as_str());
            query.push(' ');
        }
        query
    };

    println!("{}", filters);


    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, &filters);
    for statement in ast {
        println!("{:#?}", statement);
    }
   
    let x = create_csv_object(path.to_owned(), seperator);

    //println!("Valid SQL: {}", s);
    run_sql_on_csv(filters, x);
}




/// Return the file as a String
fn read_csv(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).unwrap_or_else(|error| {
        println!("Es ist ein Fehler aufgetreten!!! {}", error);
        String::new()
    });
    contents
    //let contents = fs::read_to_string(file_path)
}