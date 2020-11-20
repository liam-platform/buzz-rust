use std::error::Error;

use buzz::bee_query::BeeQueryRunner;
use buzz::catalog::StaticCatalog;
use buzz::query_planner::QueryPlanner;
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);
    Ok(())
}

fn my_handler(event: Value, _: Context) -> Result<Value, HandlerError> {
    println!("Input Event: {:?}", event);
    let catalog = StaticCatalog::new();
    let planner = QueryPlanner::new(Box::new(catalog));
    let (_, mut bee_queries) = planner.plan("payment_type".to_owned()).unwrap();
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(BeeQueryRunner::new().run(bee_queries.pop().unwrap()))
        .unwrap();
    Ok(Value::String("Ok!".to_owned()))
}
