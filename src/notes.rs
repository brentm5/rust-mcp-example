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
    pub async fn open(_db_path: &Path) -> Result<Self> {
        todo!()
    }

    pub async fn save(&self, _name: &str, _message: &str) -> Result<Note> {
        todo!()
    }

    pub async fn retrieve(&self, _id: &str) -> Result<Option<Note>> {
        todo!()
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
