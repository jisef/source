use std::collections::{HashMap, HashSet};
use sqlparser::parser::{Parser};
use sqlparser::dialect::GenericDialect;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use regex::Regex;
use crate::csv::CSV;


/// Return wheter the query is valid or not GenericDialect
pub fn validate_sql(mut query: String) -> Result<String, String> {
    let dialect = GenericDialect {}; // Define the SQL dialect
    if query.clone().chars().last().unwrap() != ';' {
        query.push(';');
    }
    match Parser::parse_sql(&dialect, &*query) {
        Ok(_) => Ok(query.to_string()), // Return the original query as a String
        Err(e) => Err(format!("SQL syntax error: {}", e)),
    }
}

//TODO: Where Statement
// TODO: COUNT ....

/// Edits the CSV Object that it only contains the needed rows and columns
pub fn run_filter_on_csv(query: String, csv: CSV) -> Result<CSV, String> {
    Parser::parse_sql(&GenericDialect {}, &query)
        .map_err(|e| format!("SQL syntax error: {}", e))?;
    
    //let keywords = vec!["select", "from", "where"];
    let mut csv = CSV {
        ..csv
    };
    let index_select = match query.find("SELECT") {
        Some(index) => {
            index
        },
        _ => panic!("WHAT THE SIGMA"),
    };

    let index_where = match query.find("WHERE") {
        Some(index) => index,
        _ => panic!("WHAT THE SIGMA"),
    };

    let mut select_statements = query[index_select..index_where].split(' ').filter(|s| !s.eq_ignore_ascii_case("select")).collect::<Vec<&str>>();
    select_statements.remove(select_statements.len() - 1);
    let mut where_statements = query[index_where..query.len()].split(' ').filter(|s| !s.eq_ignore_ascii_case("where")).collect::<Vec<&str>>();
    let removed = where_statements.remove(where_statements.len() - 1);
    where_statements.push(removed.trim());
    //where_statements.remove(where_statements.len() );

    let mut where_vec: Vec<CSV> = Vec::new();

    // Do WHERE STATEMENT
    //TODO: AND
    //TODO: LIKE, IN, BETWEEN
    //TODO: AND OR NOT
    //TODO: NULL, NOT NULL
    for mut i in 0..where_statements.len() - 1 {
        if where_statements[i + 1].eq("=") ||  where_statements[i + 1].eq("<>") ||
            where_statements[i + 1].eq(">") || where_statements[i + 1].eq("<") ||
            where_statements[i + 1].eq(">=")|| where_statements[i + 1].eq(">="){

            where_vec.push(csv.clone().apply_filter(where_statements[i], where_statements[i + 2], where_statements[i + 1])?);
        }
    } 
    
    
    // MERGE WHERE
    let mut new_rows: Vec<HashMap<String, String>> = Vec::new();
    
    for mut x in where_vec {
        new_rows.append(&mut x.rows);
    }
    csv.rows = remove_duplicate_maps(new_rows);
    
    // SELECT  STATEMENT
    for mut i in 0..select_statements.len() - 1 {
        if select_statements[i + 1].eq_ignore_ascii_case("as") && i < select_statements.len() - 1 {
            csv.rename_csv_column(select_statements[i], select_statements[i + 2]);
            i += 2;
        } else if select_statements[i].eq("*") {
            // headers stay the same
        }
    }
    Ok(csv)
}
pub fn eq_row(row1: &HashMap<String, String>, row2: &HashMap<String, String>) -> bool {
    let is_equal = row1 == row2;
    /*for (k, v) in row1.clone() {
        if !row2.get(&k) == row1.get(&k)  {
            return false;
        }
    }*/
    is_equal
    
    
}

fn hash_map<K, V>(map: &HashMap<K, V>) -> u64
where
    K: Eq + Hash,
    V: Hash,
{
    let mut s = DefaultHasher::new();

    // Hash each key-value pair in the map into the hasher
    for (key, value) in map.iter() {
        key.hash(&mut s);
        value.hash(&mut s);
    }

    // Return the final hash value
    s.finish()
}
fn remove_duplicate_maps(vec: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    let mut seen = HashSet::new();
    let mut result = Vec::with_capacity(vec.len());
    for map in vec {
        let hash = hash_map(&map);
        if seen.insert(hash) {
            result.push(map);
        }
    }

    result
}


pub fn insert_into_csv(mut csv: CSV, statement: String) -> Result<CSV, String> {
    let mut columns = Vec::new();
    let mut column_values: HashMap<String, String> = HashMap::new();
    let re = Regex::new(r"\((.*?)\)").unwrap();

    let mut is_first: bool = true;
    for cap in re.captures_iter(statement.to_string().as_str()) {
        //println!("{}", &cap[1]);
        let mut index: usize = 0;
        for mut column in &cap[1].split(',').collect::<Vec<&str>>() {
            if is_first {
                let new_column = column.trim();
                if !csv.contains_column(new_column) {
                    return Err(format!("Column not found: {}", column));
                }
                columns.push(new_column.to_string());    
            } else {
                let new_column = column.trim();
                let column_name  = match columns.get(index) {
                    Some(column_name) => column_name,
                    None => return Err(format!("Column not found: {}", column)),
                };
                column_values.insert(column_name.to_owned(), new_column.to_string());
                index += 1;
            }
            
        }
        is_first = false;
    }


    csv.add_row(column_values);

    Ok(csv)
}