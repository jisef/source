use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::ptr::null;

pub struct CSV {
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, String>>, // header and value
}

impl CSV {
    pub fn clone(&self) -> Self {
        CSV {
            headers: self.headers.clone(),
            rows: self.rows.clone(),
        }
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
    pub fn apply_filter(&self, column_name: &str, value: &str, op: &str) -> CSV {
        let mut new_rows: Vec<HashMap<String, String>> = Vec::new();
        for x in self.rows.iter() {
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
        }
        
        CSV {
            headers: self.headers.clone(),
            rows: new_rows,
        }
    }
    
    
    
}

/// Reads from the given file and Returns it as a CSV Object: headers:Vec<String>,
/// Vec<Hashmap<String(columm name), String (value)>
pub fn create_csv_object(path: String, seperator: char) -> CSV{
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

