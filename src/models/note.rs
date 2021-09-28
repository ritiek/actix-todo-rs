use chrono::{DateTime, Local};
use mongodb::{
    bson::doc,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};
use serde::{Deserialize, Serialize};
use std::{
    env,
    io::{self, BufRead},
    process,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    #[serde(rename = "_id")]
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_on: DateTime<Local>,
}

impl Note {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: "".to_string(),
            description: "".to_string(),
            completed: false,
            created_on: Local::now(),
        }
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn create(self) -> Result<Self, ()> {
        Ok(Self {
            id: self.id,
            title: self.title,
            description: self.description,
            completed: self.completed,
            created_on: self.created_on,
        })
    }

    pub fn summarize(&self) -> String {
        let created_on_fmt = self.created_on.format("%d/%m/%Y @ %I:%M:%S %p");
        format!(
            "ID: {}\nTitle: {}\nDescription: {}\n[Status: {}]\n(Created on: {})",
            self.id,
            self.title,
            self.description,
            if self.completed {
                "Completed"
            } else {
                "Pending"
            },
            created_on_fmt,
        )
    }

    pub fn save_to(
        &self,
        collection: &mut Collection<Self>,
    ) -> mongodb::error::Result<InsertOneResult> {
        collection.insert_one(self, None)
    }
}

fn show_saved_notes_from(collection: &Collection<Note>) -> mongodb::error::Result<()> {
    let saved_notes = collection.find(None, None)?;
    println!("Summary of all saved todo Notes:\n");
    for saved_note in saved_notes {
        println!("{}\n", saved_note?.summarize());
    }
    Ok(())
}

fn mark_note_task_as_completed(
    collection: &mut Collection<Note>,
    note_id: i32,
) -> mongodb::error::Result<UpdateResult> {
    collection.update_one(
        doc! { "_id": note_id },
        doc! { "$set": { "completed": true } },
        None,
    )
}

fn delete_note_task(
    collection: &mut Collection<Note>,
    note_id: i32,
) -> mongodb::error::Result<DeleteResult> {
    collection.delete_one(doc! { "_id": note_id }, None)
}

// fn add_note_to(collection: &mut Collection<Note>) -> mongodb::error::Result<InsertOneResult> {
//     let saved_notes = collection.find(None, None)?;
//     let id = saved_notes.count() + 1;
//     println!("Enter a title for the todo note:");
//     let title = read_line_from_stdin();
//     println!("Enter a description for the todo note:");
//     let description = read_line_from_stdin();
//     Note::new()
//         .with_id(id as i32)
//         .with_title(title)
//         .with_description(description)
//         .create()
//         .unwrap()
//         .save_to(collection)
// }
