use std::pin::Pin;

use crate::hcomb_manager::HCombAddress;
use arrow::record_batch::RecordBatch;
use async_trait::async_trait;
use datafusion::logical_plan::LogicalPlan;
use tokio::stream::Stream;

#[async_trait]
pub trait HCombScheduler {
    /// Notifies the hcomb that a query is starting and opens a stream of results.
    async fn schedule(
        &self,
        address: &HCombAddress,
        hbee_count: usize,
        plan: LogicalPlan,
    ) -> Pin<Box<dyn Stream<Item = RecordBatch>>>;
}

// TODO implementations:
// TODO - simple http