/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use std::path::Path;
use crate::core::objects::ScopesResp;

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
