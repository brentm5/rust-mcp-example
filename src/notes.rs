#![allow(unused_imports, dead_code, unused)]
use std::path::Path;
use std::sync::Arc;

use arrow_array::{RecordBatch, RecordBatchIterator, StringArray};
use arrow_schema::{DataType, Field, Schema};
use futures::TryStreamExt;
use lancedb::index::scalar::FtsIndexBuilder;
use lancedb::index::Index;
use lancedb::query::{ExecutableQuery, QueryBase};
use lancedb::{Connection, Table};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct Note {
    pub id: String,
    pub name: String,
    pub message: String,
}

fn note_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("message", DataType::Utf8, false),
    ]))
}

pub struct NoteStore {
    table: Table,
}

impl NoteStore {
    pub async fn open(db_path: &Path) -> Result<Self> {
        std::fs::create_dir_all(db_path)?;
        let conn: Connection = lancedb::connect(db_path.to_str().ok_or("invalid path")?)
            .execute()
            .await?;

        let table = if conn
            .table_names()
            .execute()
            .await?
            .contains(&"notes".to_string())
        {
            conn.open_table("notes").execute().await?
        } else {
            let schema = note_schema();
            let empty = RecordBatchIterator::new(std::iter::empty(), schema.clone());
            let tbl = conn.create_table("notes", empty).execute().await?;
            tbl.create_index(&["name"], Index::FTS(FtsIndexBuilder::default()))
                .execute()
                .await?;
            tbl.create_index(&["message"], Index::FTS(FtsIndexBuilder::default()))
                .execute()
                .await?;
            tbl
        };

        Ok(Self { table })
    }

    pub async fn save(&self, name: &str, message: &str) -> Result<Note> {
        let id = Uuid::new_v4().to_string();
        let schema = note_schema();
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(vec![id.as_str()])),
                Arc::new(StringArray::from(vec![name])),
                Arc::new(StringArray::from(vec![message])),
            ],
        )?;
        let reader = RecordBatchIterator::new(std::iter::once(Ok(batch)), schema);
        self.table.add(reader).execute().await?;
        Ok(Note {
            id,
            name: name.to_string(),
            message: message.to_string(),
        })
    }

    pub async fn retrieve(&self, id: &str) -> Result<Option<Note>> {
        let filter = format!("id = '{}'", id.replace('\'', "''"));
        let batches: Vec<RecordBatch> = self
            .table
            .query()
            .only_if(&filter)
            .execute()
            .await?
            .try_collect()
            .await?;
        let notes = batches_to_notes(batches)?;
        Ok(notes.into_iter().next())
    }

    pub async fn search(&self, _query: &str) -> Result<Vec<Note>> {
        todo!()
    }

    pub async fn list(&self) -> Result<Vec<Note>> {
        todo!()
    }
}

fn batches_to_notes(batches: Vec<RecordBatch>) -> Result<Vec<Note>> {
    let mut notes = Vec::new();
    for batch in batches {
        let ids = batch
            .column_by_name("id")
            .and_then(|c| c.as_any().downcast_ref::<StringArray>())
            .ok_or("missing id column")?;
        let names = batch
            .column_by_name("name")
            .and_then(|c| c.as_any().downcast_ref::<StringArray>())
            .ok_or("missing name column")?;
        let messages = batch
            .column_by_name("message")
            .and_then(|c| c.as_any().downcast_ref::<StringArray>())
            .ok_or("missing message column")?;
        for i in 0..batch.num_rows() {
            notes.push(Note {
                id: ids.value(i).to_string(),
                name: names.value(i).to_string(),
                message: messages.value(i).to_string(),
            });
        }
    }
    Ok(notes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn rt() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().unwrap()
    }

    #[test]
    fn test_save_returns_note_with_uuid() {
        rt().block_on(async {
            let dir = tempdir().unwrap();
            let store = NoteStore::open(dir.path()).await.unwrap();
            let note = store.save("test note", "hello world").await.unwrap();
            assert!(!note.id.is_empty());
            assert_eq!(note.name, "test note");
            assert_eq!(note.message, "hello world");
            assert_eq!(note.id.len(), 36);
        });
    }

    #[test]
    fn test_retrieve_by_id() {
        rt().block_on(async {
            let dir = tempdir().unwrap();
            let store = NoteStore::open(dir.path()).await.unwrap();
            let saved = store.save("my note", "content here").await.unwrap();
            let found = store.retrieve(&saved.id).await.unwrap();
            assert!(found.is_some());
            let note = found.unwrap();
            assert_eq!(note.id, saved.id);
            assert_eq!(note.name, "my note");
            assert_eq!(note.message, "content here");
        });
    }

    #[test]
    fn test_retrieve_missing_id_returns_none() {
        rt().block_on(async {
            let dir = tempdir().unwrap();
            let store = NoteStore::open(dir.path()).await.unwrap();
            let result = store.retrieve("nonexistent-id").await.unwrap();
            assert!(result.is_none());
        });
    }
}
