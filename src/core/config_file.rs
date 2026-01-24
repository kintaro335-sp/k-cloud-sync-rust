/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use std::fs;
use std::io;
use std::string::String;
use serde::{de::Error};
use serde_json::{Result};
pub use crate::core::objects;

pub fn exists_file(file_name: &String) -> io::Result<bool> {
  let exist = fs::exists(file_name)?;
  Ok(exist)
}

pub fn load_config(file_name: &String) -> Result<objects::Configfile> {
  let raw_json: String = fs::read_to_string(file_name).expect("Unable to read file");
  let info: objects::Configfile = serde_json::from_str(&raw_json)?;
  let mut valid: bool = true;
  if info.base_url.trim().is_empty() {
    valid = false;
  }

  if info.api_key.trim().is_empty() {
    valid = false;
  }

  if info.dirs.is_empty() {
    valid = false;
  }

  for dir in info.dirs.iter() {
    if dir.remote_path.trim().is_empty() {
      valid = false;
    }
    if dir.local_path.trim().is_empty() {
      valid = false;
    }
    if dir.sync_mode.trim().is_empty() {
      valid = false;
    }
  }

  match valid {
    true => return Ok(info),
    false => return Err(Error::custom("Invalid config file")),
      
  }
}
