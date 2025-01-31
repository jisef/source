use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use sqlparser::ast::IndexType::Hash;

pub struct CSV {
    headers: Vec<String>,
    rows: Vec<HashMap<String, String>>, // header and value
}


pub fn create_csv_Object(path: String, seperator: char) -> CSV{
    let mut headers: Vec<String> = Vec::new();
    let mut rows: Vec<HashMap<String, String>> = Vec::new();
    
    
    let file = std::fs::File::open(path).unwrap();
    let mut br = BufReader::new(file);
    let mut is_fist: bool = true;
    for line_result in br.lines() {
        match line_result {
            Ok(line) => {
                if is_fist {
                    headers = line.split(seperator).map(|s| s.to_string()).collect();
                    is_fist = false;
                } else {
                    let mut map: HashMap<String, String> = HashMap::new();
                    for (header, element) in headers.iter().zip(line.split(seperator)) {
                        map.insert(header.to_string(), element.to_owned());
                    }
                    rows.push(map)

                }
            }
            _ => {break;}
        }
    }
    
    
    CSV {
        headers: headers,
        rows: rows,
    }
}

