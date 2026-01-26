/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use std::path::Path;
use crate::core::objects::{ScopesResp, FileList};

pub fn create_path(virtual_path: &str, dir_name: &str) -> String {
  return Path::new(virtual_path).join(dir_name).display().to_string();
  // return format!("{}/{}", virtual_path, dir_name);
}

pub fn is_authorized(scopes: &ScopesResp) -> bool {
  let scopes_list = scopes.scopes.clone();
  let mut scopes_needed_found: u8 = 0;

  for scope_item in scopes_list {
    if scope_item == "files:read" || scope_item == "files:create" {
      scopes_needed_found += 1
    }
  }
  scopes_needed_found >= 2
}

pub fn exists_file_remote(files_list: &FileList, file_name: &String) -> bool {
  let list = &files_list.list;

  for f in list {
    if f.name == *file_name {
      return true
    }
  }

  false
}

pub fn calc_file_uploaded(bytes_uploaded: u64, size: u64) -> f32 {
  if bytes_uploaded == 0 {
    return 0_f32
  }
  let bytes_uploaded_f:f32 = bytes_uploaded as f32;
  let size_f:f32 = size as f32;
  let percentage_decimal: f32 = bytes_uploaded_f / size_f;
  let percentage = percentage_decimal * 100_f32;
  percentage
}
