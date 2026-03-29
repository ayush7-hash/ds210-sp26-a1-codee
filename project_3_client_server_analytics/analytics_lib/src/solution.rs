use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn evaluate_condition(row: &Row, condition: &Condition, dataset: &Dataset) -> bool {
        match condition {
            Condition::Equal(col, val) => {
                let index = dataset.column_index(col);
                row.get_value(index) == val
            }
            Condition::Not(inner) => {
                !evaluate_condition(row, inner, dataset)
            }
            Condition::And(left, right) => {
                evaluate_condition(row, left, dataset) && evaluate_condition(row, right, dataset)
            }
            Condition::Or(left, right) => {
                evaluate_condition(row, left, dataset) || evaluate_condition(row, right, dataset)
            }
        }
    }

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut filtered = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() {
        if evaluate_condition(row, filter, dataset) {
            filtered.add_row(row.clone());
        }
    }
    return filtered
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let col_name = match aggregation {
        Aggregation::Count(col)   => col,
        Aggregation::Sum(col)     => col,
        Aggregation::Average(col) => col,
    };

    dataset
        .into_iter()
        .map(|(key, group)| {
            let value = match aggregation {
                Aggregation::Count(_) => Value::Integer(group.len() as i32),
                Aggregation::Sum(_) => {
                    let col_index = group.column_index(col_name);
                    let sum: i32 = group.iter()
                        .map(|row| row.get_value(col_index))
                        .filter_map(|val| match val {
                            Value::Integer(i) => Some(i),
                            _ => None,
                        })
                        .sum();
                    Value::Integer(sum)
                }
                Aggregation::Average(_) => {
                    let col_index = group.column_index(col_name);
                    let sum: i32 = group.iter()
                        .map(|row| row.get_value(col_index))
                        .filter_map(|val| match val {
                            Value::Integer(i) => Some(i),
                            _ => None,
                        })
                        .sum();
                    let count = group.len() as i32;
                    Value::Integer(sum / count)
                }
            };
            (key, value)
        })
        .collect()
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}