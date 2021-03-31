use std::path::PathBuf;
use std::fs::{File, create_dir};

use serde_json::{Map, Value};
use clap::crate_name;
use directories::BaseDirs;

use crate::Res;

const FILE_NAME: &str = "data.json";

pub struct FileAccess {
    path: Option<PathBuf>
}

impl FileAccess {
    pub fn new() -> Self {
        if let Ok(found_path) = Self::get_or_create_dir() {
            return FileAccess { path: Some(found_path) }
        }

        FileAccess { path: None }
    }

    pub fn read(&self) -> Res<Value> {
        match &self.path {
            Some(path_buf) => {
                let path = path_buf.as_path();

                let val;
                let file_path = path.join(FILE_NAME);
        
                if file_path.exists() {
                    println!("Found file... reading");
        
                    let file = File::open(file_path)?;
                    val = serde_json::from_reader(file)?;
                } else {
                    println!("Creating new file at {:?}", file_path);
        
                    let map = Map::new();
                    self.write(&map)?;
                    val = Value::Object(map);
                }

                return Ok(val)
            },
            _ => return Err(Box::from("No path!"))
        }
    }

    pub fn write(&self, val: &Map<String, Value>) -> Res<()> {
        match &self.path {
            Some(path_buf) => {
                let path = path_buf.as_path();

                let file_path = path.join(FILE_NAME);
                let file = File::create(file_path)?;
            
                serde_json::to_writer_pretty(file, val)?;
            
                return Ok(())
            },
            _ => return Err(Box::from("No path!"))
        }
    }

    fn get_or_create_dir() -> Res<PathBuf> {
        if let Some(base) = BaseDirs::new() {
            let data_dir = base.data_dir().join(crate_name!());
    
            if !data_dir.exists() {
                create_dir(&data_dir)?;
            }
    
            Ok(data_dir)
        } else {
            Err(Box::from("Could not create directory"))
        }
    }
}
