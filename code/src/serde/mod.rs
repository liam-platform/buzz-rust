pub mod from_proto;
pub mod to_proto;

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use std::sync::Arc;

    use crate::catalog::SizedFile;
    use crate::datasource::ParquetTable;
    use crate::protobuf;
    use arrow::datatypes::{DataType, Field, Schema};
    use datafusion::execution::context::ExecutionContext;
    use datafusion::logical_plan::LogicalPlan;
    use datafusion::logical_plan::{col, count};

    #[test]
    fn roundtrip() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let parquet_table = mock_table();

        let source_df = ExecutionContext::new().read_table(Arc::new(parquet_table))?;

        let source_plan = source_df.to_logical_plan();

        let proto: protobuf::LogicalPlanNode = (&source_plan).try_into()?;

        let transfered_plan: LogicalPlan = (&proto).try_into()?;

        assert_eq!(
            format!("{:?}", source_plan),
            format!("{:?}", transfered_plan)
        );

        Ok(())
    }

    #[test]
    fn roundtrip_aggregate() -> Result<(), Box<dyn std::error::Error>> {
        let parquet_table = mock_table();

        let source_df = ExecutionContext::new()
            .read_table(Arc::new(parquet_table))?
            .aggregate(vec![col("state")], vec![count(col("state"))])?;

        let source_plan = source_df.to_logical_plan();

        let proto: protobuf::LogicalPlanNode = (&source_plan).try_into()?;

        let transfered_plan: LogicalPlan = (&proto).try_into()?;

        assert_eq!(
            format!("{:?}", source_plan),
            format!("{:?}", transfered_plan)
        );

        Ok(())
    }

    fn mock_table() -> ParquetTable {
        let schema = Schema::new(vec![
            Field::new("id", DataType::Int32, false),
            Field::new("first_name", DataType::Utf8, false),
            Field::new("last_name", DataType::Utf8, false),
            Field::new("state", DataType::Utf8, false),
            Field::new("salary", DataType::Int32, false),
        ]);

        ParquetTable::new(
            "south-pole-1".to_owned(),
            "santa".to_owned(),
            vec![SizedFile {
                key: "gift1".to_owned(),
                length: 1,
            }],
            Arc::new(schema),
        )
    }
}
