/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */

pub fn create_path(virtual_path: &str, dir_name: &str) -> String {
  return format!("{}/{}", virtual_path, dir_name);
}

