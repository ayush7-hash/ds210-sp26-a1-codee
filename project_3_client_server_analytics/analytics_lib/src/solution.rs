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

    filtered
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let group_index = dataset.column_index(group_by_column);;
    let mut grouped_data: HashMap<Value, Dataset> = HashMap::new();

    for row in dataset.iter() {
        let key = row.get_value(group_index).clone();;

        if !grouped_data.contains_key(&key) {
            grouped_data.insert(key.clone(), Dataset::new(dataset.columns().clone()));
        }

        grouped_data.get_mut(&key).unwrap().add_row(row.clone());
    }

    grouped_data
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
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