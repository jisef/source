use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

pub fn validate_sql(query: String) -> Result<String, String> {
    let dialect = PostgreSqlDialect {}; // Define the SQL dialect
    match Parser::parse_sql(&dialect, &*query) {
        Ok(_) => Ok(query.to_string()), // Return the original query as a String
        Err(e) => Err(format!("SQL syntax error: {}", e)),
    }
}