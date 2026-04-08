use std::{error::Error, fs::File, io::{BufReader, BufWriter, Read, Write}, path::Path};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::core::error::IoError;


pub fn save_to_json<'a, T: Serialize>(file_path: &'a Path, data: &'a T) -> Result<(), IoError<'a>>{
    let e = Err(IoError::WriteError { path: file_path });
    let Ok(file) =  File::create(file_path) else { return e };
    let mut writer = BufWriter::new(file);
    let Ok(_) = serde_json::to_writer_pretty(&mut writer, data) else { return e };
    let Ok(_) = writer.flush() else { return e };
    Ok(())
}

pub fn load_from_json<T: for<'a> Deserialize<'a>>(file_path: &'_ Path) -> Result<T, IoError<'_>>{
    let e = Err(IoError::ReadError { path: file_path });
    let Ok(file) = File::open(file_path) else { return e };
    let mut reader = BufReader::new(file);
    let Ok(data) = serde_json::from_reader(&mut reader) else { return e };
    Ok(data)
}