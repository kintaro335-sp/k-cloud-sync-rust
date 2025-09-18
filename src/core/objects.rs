use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Dirsync {
  pub remote_path: String,
  pub local_path: String,
  pub sync_mode: String,
}

#[derive(Debug, Deserialize)]
pub struct Configfile {
  pub base_url: String,
  pub api_key: String,
  pub dirs: Vec<Dirsync>,
}

#[derive(Debug, Deserialize)]
#[warn(non_snake_case)]
pub struct User {
  pub sessionId: String,
  pub userId: String,
  pub username: String,
  // pub isadmin: bool
}

#[derive(Debug, Deserialize)]
pub struct File {
  pub name: String,
  pub r#type: String,
  // pub size: i128,
  // pub extension: String,
  // pub mime_type: String
}

#[derive(Debug, Deserialize)]
pub struct FileList {
  pub list: Vec<File>
}

#[derive(Debug, Deserialize)]
pub struct FileProperties {
  // pub name: String,
  pub r#type: String,
  // pub size: i128,
  // pub extension: String,
  // pub mime_type: String
}
