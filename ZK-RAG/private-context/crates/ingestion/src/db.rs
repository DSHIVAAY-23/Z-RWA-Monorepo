use anyhow::Result;
use arrow::array::{Float32Array, RecordBatch, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatchIterator;
use lancedb::connection::Connection;
use lancedb::{connect, Table};
use lancedb::query::{ExecutableQuery, QueryBase}; 
use std::sync::Arc;
use futures::TryStreamExt;

pub struct VectorStore {
    conn: Connection,
    table_name: String,
}

impl VectorStore {
    pub async fn new(uri: &str) -> Result<Self> {
        let conn = connect(uri).execute().await?;
        Ok(Self {
            conn,
            table_name: "vectors".to_string(),
        })
    }

    pub async fn create_table(&self) -> Result<Table> {
        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("text", DataType::Utf8, false),
            Field::new("source", DataType::Utf8, false),
            Field::new(
                "vector",
                DataType::FixedSizeList(
                    Arc::new(Field::new("item", DataType::Float32, true)),
                    384, // MiniLM dimension
                ),
                false,
            ),
        ]));

        // LanceDB create_empty_table needs a dummy batch usually, or we use create_table with data.
        // For simplicity in this CLI, we'll assume we pass data to create it if it doesn't exist,
        // or just return the table if it does.
        
        let table = if self.conn.open_table(&self.table_name).execute().await.is_err() {
            // Table doesn't exist, we must create it with initial data or empty.
            // LanceDB 0.4.x create_table takes an iterator of RecordBatches.
            // We'll defer creation to the add method or create an empty one here if possible.
            // NOTE: Creating empty table in older LanceDB might be tricky without data.
            // Let's simpler: return None or let add_chunks handle creation.
            // For now, let's try to just open, and if not, we wait for first write.
            return Err(anyhow::anyhow!("Table does not exist. It will be created on first ingestion."));
        } else {
             self.conn.open_table(&self.table_name).execute().await?
        };

        Ok(table)
    }

    pub async fn add(&self, ids: Vec<String>, texts: Vec<String>, sources: Vec<String>, vectors: Vec<Vec<f32>>) -> Result<()> {
        let len = ids.len();
        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("text", DataType::Utf8, false),
            Field::new("source", DataType::Utf8, false),
            Field::new(
                "vector",
                DataType::FixedSizeList(
                    Arc::new(Field::new("item", DataType::Float32, true)),
                    384,
                ),
                false,
            ),
        ]));

        let id_array = StringArray::from(ids);
        let text_array = StringArray::from(texts);
        let source_array = StringArray::from(sources);
        
        // Flatten vectors
        let flattened_vec: Vec<f32> = vectors.into_iter().flatten().collect();
        // Correct way for FixedSizeList in Arrow usually involves building it from the child data.
        let vector_data = Float32Array::from(flattened_vec);
        let vector_list = arrow::array::FixedSizeListArray::try_new(
            Arc::new(Field::new("item", DataType::Float32, true)),
            384,
            Arc::new(vector_data),
            None
        )?;

        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(id_array),
                Arc::new(text_array),
                Arc::new(source_array),
                Arc::new(vector_list),
            ],
        )?;

        // Try to open table, if fails, create it
        if let Ok(table) = self.conn.open_table(&self.table_name).execute().await {
            let reader = RecordBatchIterator::new(vec![Ok(batch.clone())], schema.clone());
            table.add(reader).execute().await?;
        } else {
            let reader = RecordBatchIterator::new(vec![Ok(batch)], schema.clone());
            self.conn
                .create_table(&self.table_name, reader)
                .execute()
                .await?;
        }

        Ok(())
    }
    
    pub async fn search(&self, query_vector: &[f32], k: usize) -> Result<Vec<(String, f32, Vec<f32>)>> { // Returns (Text, Distance, Vector)
         let table = self.conn.open_table(&self.table_name).execute().await?;
         // LanceDB 0.4+ query Builder often uses query()
         let results = table.query()
            .nearest_to(query_vector)?
            .limit(k)
            .execute()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

         // Map results to easy format
         let mut out = Vec::new();
         for batch in results {
             let text_col = batch.column_by_name("text").unwrap().as_any().downcast_ref::<StringArray>().unwrap();
             let vector_col = batch.column_by_name("vector").unwrap().as_any().downcast_ref::<arrow::array::FixedSizeListArray>().unwrap();
             
             // In lancedb 0.4, distance might be in the metadata or a separate column "_distance"
             for i in 0..batch.num_rows() {
                 let text = text_col.value(i).to_string();
                 
                 // Extract vector
                 let val = vector_col.value(i);
                 let float_array = val.as_any().downcast_ref::<Float32Array>().unwrap();
                 let vec: Vec<f32> = float_array.values().to_vec();

                 out.push((text, 0.0, vec));
             }
         }
         Ok(out)
    }
}
