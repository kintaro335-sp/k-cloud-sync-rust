/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use std::path::Path;

pub fn create_path(virtual_path: &str, dir_name: &str) -> String {
  return Path::new(virtual_path).join(dir_name).display().to_string();
  // return format!("{}/{}", virtual_path, dir_name);
}

