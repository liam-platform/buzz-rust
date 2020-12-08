pub mod bee_query;
pub mod catalog;
pub mod dataframe_ops;
pub mod datasource;
pub mod error;
pub mod execution_plan;
pub mod flight_client;
pub mod flight_service;
pub mod hive_query;
pub mod query_planner;
pub mod range_cache;
pub mod results_service;
pub mod s3;
pub mod serde;

// include the generated protobuf source as a submodule
#[allow(clippy::all)]
pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/buzz.protobuf.rs"));
}
