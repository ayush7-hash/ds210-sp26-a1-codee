extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::query::Query;
use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    use analytics_lib::query::{Aggregation, Condition};
    use analytics_lib::dataset::Value;
    let tokens: Vec<String> = input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("!", " ! ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut pos = 0;

    // Parse condition recursively
    fn parse_condition(tokens: &Vec<String>, pos: &mut usize) -> Condition {
        use analytics_lib::query::Condition;
        use analytics_lib::dataset::Value;

        if tokens[*pos] == "(" {
            *pos += 1; // consume "("

            if tokens[*pos] == "!" {
                *pos += 1; // consume "!"
                let inner = parse_condition(tokens, pos);
                *pos += 1; // consume ")"
                return Condition::Not(Box::new(inner));
            }

            let left = parse_condition(tokens, pos);
            let operator = tokens[*pos].clone();
            *pos += 1; // consume AND/OR

            let right = parse_condition(tokens, pos);
            *pos += 1; // consume ")"

            if operator == "AND" {
                return Condition::And(Box::new(left), Box::new(right));
            } else {
                return Condition::Or(Box::new(left), Box::new(right));
            }
        } else {
            // Simple equality: col == "value"
            let column = tokens[*pos].clone();
            *pos += 1; // consume column name
            *pos += 1; // consume "=="
            let raw_value = tokens[*pos].trim_matches('"').to_string();
            *pos += 1; // consume value

            // Try parsing as integer, otherwise string
            let value = if let Ok(n) = raw_value.parse::<i32>() {
                Value::Integer(n)
            } else {
                Value::String(raw_value)
            };

            return Condition::Equal(column, value);
        }
    }

    // Expect "FILTER"
    assert_eq!(tokens[pos], "FILTER");
    pos += 1;

    // Parse condition
    let condition = parse_condition(&tokens, &mut pos);

    // Expect "GROUP"
    assert_eq!(tokens[pos], "GROUP");
    pos += 1;

    // Expect "BY"
    assert_eq!(tokens[pos], "BY");
    pos += 1;

    // Parse group by column
    let group_by = tokens[pos].clone();
    pos += 1;

    // Parse aggregation
    let aggregation_type = tokens[pos].clone();
    pos += 1;
    let aggregation_column = tokens[pos].clone();

    let aggregation = match aggregation_type.as_str() {
        "COUNT" => Aggregation::Count(aggregation_column),
        "SUM" => Aggregation::Sum(aggregation_column),
        "AVERAGE" => Aggregation::Average(aggregation_column),
        _ => panic!("Unknown aggregation type: {}", aggregation_type),
    };

    return Query::new(condition, group_by, aggregation);
}


// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}