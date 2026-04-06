use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    fn evaluate(row: &Row, condition: &Condition, dataset: &Dataset) -> bool {
        match condition {
            Condition::Equal(col, val) => row.get_value(dataset.column_index(col)) == val,
            Condition::Not(inner) => !evaluate(row, inner, dataset),
            Condition::And(l, r) => evaluate(row, l, dataset) && evaluate(row, r, dataset),
            Condition::Or(l, r) => evaluate(row, l, dataset) || evaluate(row, r, dataset),
        }
    }
    let mut result = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() {
        if evaluate(row, filter, dataset) {
            result.add_row(row.clone());
        }
    }
    return result;
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let mut result: HashMap<Value, Dataset> = HashMap::new();
    let col_index = dataset.column_index(group_by_column);
    let columns = dataset.columns().clone();
    for row in dataset.into_iter() {
        let key = row.get_value(col_index).clone();
        let group = result.entry(key).or_insert_with(|| Dataset::new(columns.clone()));
        group.add_row(row);
    }
    return result;
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result: HashMap<Value, Value> = HashMap::new();
    for (key, group) in dataset {
        let value = match aggregation {
            Aggregation::Count(_) => {
                Value::Integer(group.len() as i32)
            }
            Aggregation::Sum(column_name) => {
                let mut sum = 0;
                let col_index = group.column_index(column_name);
                for row in group.iter() {
                    if let Value::Integer(v) = row.get_value(col_index) {
                        sum += v;
                    }
                }
                Value::Integer(sum)
            }
            Aggregation::Average(column_name) => {
                let mut sum = 0;
                let count = group.len() as i32;
                let col_index = group.column_index(column_name);
                for row in group.iter() {
                    if let Value::Integer(v) = row.get_value(col_index) {
                        sum += v;
                    }
                }
                Value::Integer(sum / count)
            }
        };
        result.insert(key, value);
    }
    return result;
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