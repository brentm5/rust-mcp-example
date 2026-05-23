use std::path::PathBuf;

use clap::{Args, Subcommand};

use crate::notes::NoteStore;

#[derive(Args, Debug)]
pub struct NotesArgs {
    /// Override the database directory (default: ~/.local/state/rust-mcp-example)
    #[arg(long, global = true)]
    pub db_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: NotesCommands,
}

#[derive(Subcommand, Debug)]
pub enum NotesCommands {
    /// Save a new note
    Save(SaveArgs),
    /// Retrieve a note by ID
    Retrieve(RetrieveArgs),
    /// Search notes by keyword
    Search(SearchArgs),
    /// List all notes
    List,
}

#[derive(Args, Debug)]
pub struct SaveArgs {
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub message: String,
}

#[derive(Args, Debug)]
pub struct RetrieveArgs {
    #[arg(long)]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    #[arg(long)]
    pub query: String,
}

pub fn run(args: &NotesArgs) {
    let db_path = args
        .db_path
        .clone()
        .unwrap_or_else(crate::notes::default_db_path);
    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.block_on(async {
        let store = match NoteStore::open(&db_path).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error opening notes database: {e}");
                std::process::exit(1);
            }
        };
        match &args.command {
            NotesCommands::Save(a) => match store.save(&a.name, &a.message).await {
                Ok(note) => println!("{}", note.id),
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            },
            NotesCommands::Retrieve(a) => match store.retrieve(&a.id).await {
                Ok(Some(note)) => println!("{note}"),
                Ok(None) => println!("not found"),
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            },
            NotesCommands::Search(a) => match store.search(&a.query).await {
                Ok(notes) if notes.is_empty() => println!("no results"),
                Ok(notes) => {
                    for note in &notes {
                        println!("{note}\n---");
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            },
            NotesCommands::List => match store.list().await {
                Ok(notes) if notes.is_empty() => println!("no notes"),
                Ok(notes) => {
                    for note in &notes {
                        println!("{note}\n---");
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            },
        }
    });
}
