use sqlparser::parser::Parser;
use sqlparser::dialect::GenericDialect;


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