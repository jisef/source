mod print;
mod sql;
mod args;
mod csv;

use std::fmt::format;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use crate::csv::create_csv_object;
use crate::print::print_pretty_csv;
use crate::sql::{insert_into_csv, run_filter_on_csv};
use std::process::exit;
use std::ptr::null;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let matches = args::get_args();

    let path = matches
        .get_one::<PathBuf>("filename")
        .ok_or("Filename is required!")?;
    let path = path.to_str().ok_or("Filename is not valid!")?;
    
    // defualt value is set
    let seperator: char = *matches
        .get_one::<char>("seperator")
        .expect("Default value is + ,");

    let filters: Vec<String> = matches.get_many("sql").unwrap_or_default().cloned().collect();
    let bool = filters.len() == 0 && matches.contains_id("sql");
    let filters = if bool {
        println!("SQL Query: (GenericDialect)");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        input = input.to_string();
        input
    } else {
        //filters.iter().map(String::from).collect();
        let mut query: String = String::new(); // guaranteed it's not null
        for i in 0..filters.len() {
            query.push_str(filters[i].as_str());
            query.push(' ');
        }
        query
    };

    println!("{}", filters);


    let mut csv = match create_csv_object(path.to_owned(), seperator) {
        Ok(csv) => csv,
        Err(e) => {
            println!("CSV could not be parsed: {}", e.to_string());
            exit(1);
        }
    };

    // INSERT --------------------------------------------------------------------------------------
    if matches.contains_id("insert") {
        match matches.get_one::<String>("insert") {
            Some(statement) => {
                match insert_into_csv(csv.clone(), statement.to_owned()) {
                    Ok(x) => {
                        csv = x;
                        println!("INSERT OK");
                    },
                    Err(e) => {println!("{}", e); exit(1)}
                };
            },
            None => {}
        };
    }

    // FILTER --------------------------------------------------------------------------------------
    if matches.contains_id("sql") {
        csv = match run_filter_on_csv(filters, csv.clone()) {
            Ok(x) => {x}
            Err(e) => {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
            }
        };
    }

    
    // PRINT ---------------------------------------------------------------------------------------
    print_pretty_csv(&csv);


    // EXPORT --------------------------------------------------------------------------------------
    if matches.contains_id("filepath export") {
        let path = match matches.get_one::<String>("filepath export") {
            Some(path) => path,
            None => {&args::get_filepath()}
        };
        match csv.export_file(seperator, path) {
            Ok(x) => {println!("Export into {} was successful!", x); }
            Err(e) => {println!("Export failed: '{}'", e);}
        }
    }

    Ok(())
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