use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use colored::Colorize;
use crate::args;

/// represents a csv-file
/// headers: stores the column names - sorted
/// rows: stores the values in a vec of hashmaps as column name and value - unsorted
#[derive(Clone)]
pub struct CSV {
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, String>>,
}

impl CSV {
    pub fn clone(&self) -> Self {
        CSV {
            headers: self.headers.clone(),
            rows: self.rows.clone(),
        }
    }

    /// Writes a csv-object into a csv file
    pub fn export_file(&self, seperator: char, filepath: &String) -> Result<String, String> {
        let mut path = Path::new(filepath);
        let mut build = String::new();
        if path.is_dir() {
            build = filepath.to_owned() + "/" + &*args::get_filepath();
            path = Path::new(&build).parent().unwrap();
        }

        let file = File::create(path).or(Err(format!("Error exporting the file: {}", filepath)))?;
        let mut writer = BufWriter::new(file);
        let last = self.headers.last().unwrap();
        if self.rows.len() as i32 == 0 {
            return Err("CSV is empty".to_string());
        }
        let last_row = &self.rows.last().unwrap();
        
        for head in &self.headers {
            let last = self.headers.last().unwrap();
            
            if last == head {
                write!(writer, "{}", head).unwrap();
            } else {
                write!(writer, "{}{}", head, seperator).unwrap();
            }
        }
        write!(writer, "\n").unwrap();
        
        
        for row in &self.rows {
            for head in &self.headers {
                let last = row.get(self.headers.last().unwrap()).unwrap();
                let item = row.get(head).unwrap();
                if last == item {
                    write!(writer, "{}", item).unwrap();
                } else {
                    write!(writer, "{}{}", item, seperator).unwrap();
                }
            }
            if last_row != &row {
                write!(writer, "\n").unwrap();
            }
        }
        
        writer.flush().unwrap();
        Ok(path.to_str().unwrap().to_string())
    }
    

    /// Replaces the column of a object CSV and puts it in the first place in headers
    /// RETURNS: CSV
    pub fn rename_csv_column(&mut self, base_column: &str, new_column: &str) {
        // rename header
        self.headers.iter_mut().for_each(|h| {*h = h.replace(base_column, new_column);});

        // rename rows
        for mut x in &self.rows {
            for mut y in x.keys() {
                if y.eq_ignore_ascii_case(base_column) {
                    y = &new_column.to_string();
                }
            }
        }

        self.move_front(new_column);
        /*let i = self.headers.iter().position(|x| x == new_column).expect(&format!("Column {} not found", new_column));
        let removed_item = self.headers.remove(i);
        self.headers.insert(0, removed_item);*/
        
    }
    /// Moves the column name to the front
    fn move_front(&mut self, new_column: &str) {
        let i = self.headers.iter().position(|x| x.eq(new_column)).expect(&format!("Column {} not found", new_column));
        let removed_item = self.headers.remove(i);
        self.headers.insert(0, removed_item);
    }
    
    
    /// Applys basic filters like =, <>, <, >, <=, >=
    /// Returns the CSV with the applied filter
    pub fn apply_filter(&self, column_name: &str, value: &str, op: &str) -> Result<CSV, String> {
        let mut new_rows: Vec<HashMap<String, String>> = Vec::new();
        for x in self.rows.iter() {
            if x.contains_key(column_name) {
                if op.eq("=") {
                    if x[column_name].eq_ignore_ascii_case(value) {
                        new_rows.push(x.clone());
                    }
                } else if op.eq("<>") {
                    if !x[column_name].eq_ignore_ascii_case(value) {
                        new_rows.push(x.clone());
                    }
                } else if op.eq("<") {
                    if x[column_name].cmp(&value.to_string()) == Ordering::Less {
                        new_rows.push(x.clone());
                    }
                } else if op.eq(">") {
                    if x[column_name].cmp(&value.to_string()) == Ordering::Greater {
                        new_rows.push(x.clone());
                    }
                } else if op.eq("<=") {
                    if x[column_name].cmp(&value.to_string()) == Ordering::Equal ||
                        x[column_name].cmp(&value.to_string()) == Ordering::Less {

                    }
                } else if op.eq(">=") {
                    if x[column_name].cmp(&value.to_string()) == Ordering::Equal ||
                        x[column_name].cmp(&value.to_string()) == Ordering::Greater {
                    }
                }
            } else {
                return Err(format!("Column '{}' not found!", column_name).red().to_string());
            }
        }
        
        Ok(CSV {
            headers: self.headers.clone(),
            rows: new_rows,
        })
    }
    
    
    /// Adds a row to the csv if a HashMap doesn't contain all headers they are added with empty values 
    pub fn add_row(&mut self, map: HashMap<String, String>) {
        let mut map = map;
        if !self.rows.contains(&map) {
            for x in self.headers.clone() {
                if !map.contains_key(&x) {
                    map.insert(x, String::from(""));
                }
            }
            
            
            self.rows.push(map);
        }
    }
    
    pub fn contains_column(&self, column: &str) -> bool {
        self.headers.contains(&column.to_string())
    }
    
}

/// Reads from the given file and Returns it as a CSV Object: headers:Vec<String>,
/// Vec<Hashmap<String(columm name), String (value)>
pub fn create_csv_object(path: String, seperator: char) -> Result<CSV, String> {
    let mut headers: Vec<String> = Vec::new();
    let mut rows: Vec<HashMap<String, String>> = Vec::new();
    
    
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    let br = BufReader::new(file);
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
    
    
    Ok(CSV {
        headers: headers,
        rows: rows,
    })
}







