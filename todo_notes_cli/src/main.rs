
use std::{fs::{self, File}, io::{BufWriter, Write}, time::SystemTime};
use chrono::DateTime;
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
            println!("Please insert a command. You can use help the see the comannd list");
            return Ok(());
        }

        match args[1].as_str() {
            "help" => println!("help: To view the command list.\nfpath: To view the file path of the json.\nadd: To add a new note to the json file. \n"),
            "path" => println!("The file path is {:?}", TODO_PATH),
            "list" => {
                show_todo_list()?;
            },
            "add" => {
                if create_directory()? {
                    let text_to_add = args[2..].join(" ");
                    add_note( text_to_add.as_str())?;
                }
                return Ok(());
            },
            "done" => {
                let id = args[2].as_str();
                mark_as(MarkAs::DONE, id)?;
            }
            "delete" => {
                let id = args[2].as_str();
                mark_as(MarkAs::DELETED, id)?;
            }
            _ => println!("Unknown command"),
        };

    Ok(())
}

static TODO_PATH: &str = "./todos.json";

#[derive(PartialEq)]
enum MarkAs {
    DONE,
    DELETED,
}

#[derive(Serialize, Deserialize)]
struct Note {
    id: Uuid,
    text: String,
    created_at: String,
    is_done: bool,
    done_at: String,
    is_deleted: bool,
    deleted_at: String,
}

impl Note {
    fn new(text: String) -> Self{
        Self {
            id: Uuid::new_v4(),
            text: text,
            created_at: get_system_date_string(),
            is_done: false,
            done_at: "".to_string(),
            is_deleted: false,
            deleted_at: "".to_string()
        }
    }
}

impl Default for Note {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            text: "Esto es un test".to_string(),
            created_at: get_system_date_string(),
            is_done: false,
            done_at: "".to_string(),
            is_deleted: false,
            deleted_at: "".to_string()
        }
    }
}

fn show_todo_list() -> Result<()> {
    let data: String = fs::read_to_string(TODO_PATH).unwrap_or_default();
    println!("{}", data);
    Ok(())
}

fn mark_as(mark_as: MarkAs, id: &str) -> Result<()>{

    let data: String = fs::read_to_string(TODO_PATH).unwrap_or_default();
    let mut data_vec: Vec<Note> = if data.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&data)?
    };

    let note= data_vec.iter_mut().find(|note|note.id.to_string() == id);

    if let Some(note) = note {
        if mark_as == MarkAs::DONE {
            note.is_done = true;
            note.done_at = get_system_date_string();
            println!("Note ID: {} mark as done correctly ", id);
        } else {
            note.is_deleted = true;
            note.deleted_at = get_system_date_string();
            println!("Note ID: {} mark as deleted correctly ", id);
        }
    } else {
        println!("Note ID: {} not found", id);
    }

    let file = File::create("todos.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &data_vec)?;
    writer.flush()?;

    Ok(())
}

fn add_note(text: &str) -> Result<()> {
    println!("Adding note...");
    let _note: Note = Note::new(text.to_string());
    let data: String = fs::read_to_string(TODO_PATH).unwrap_or_default();

    let mut data_vec: Vec<Note> = if data.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&data)?
    };

    data_vec.push(_note);

    let file = File::create("todos.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &data_vec)?;
    writer.flush()?;

    println!("Note added correctly");

    Ok(())
}

fn get_system_date_string() -> String {
    let datetime: DateTime<Utc> = SystemTime::now().into();
    return datetime.format("%d/%m/%Y %T").to_string()
}

fn create_directory() -> Result<bool> {
    match fs::metadata(TODO_PATH) {
        Ok(_) => {
            println!("The path already exists. Skipping...");
            Ok(true)
        }
        Err(_) => {
            fs::create_dir(TODO_PATH)?;
            println!("The JSON file isn't exists. Creating...");
            Ok(true)
        }
    }
}