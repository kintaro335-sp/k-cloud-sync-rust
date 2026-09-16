/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Dirsync {
  pub remote_path: String,
  pub local_path: String,
  pub sync_mode: String,
}

fn default_jobs_num() -> u16 {
  1
}

#[derive(Debug, Deserialize)]
pub struct Configfile {
  pub base_url: String,
  pub api_key: String,
  pub dirs: Vec<Dirsync>,
  #[serde(default = "default_jobs_num")]
  pub jobs: u16,
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
  pub size: usize,
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

#[derive(Debug, Deserialize)]
pub struct ExistFile {
  pub exists: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SizeBody {
  pub size: u64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScopesResp {
  pub r#type: String,
  pub scopes: Vec<String>
}

#[derive(Debug)]
pub struct FileObj {
  pub action: String,
  pub remote_path: String,
  pub local_path: String,
  pub size: usize,
  pub r#type: String
}
