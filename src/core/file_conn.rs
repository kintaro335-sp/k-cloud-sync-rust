/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use std::{fs::read_dir, fs::File, fs::metadata};
use std::path::Path;
use std::io;
// use std::io::copy;
// use filesize::PathExt;

pub fn file_list(path: &str) -> Vec<String> {
  let mut list : Vec<String> = Vec::new();

  for entry in read_dir(path).unwrap() {
    let path = entry.unwrap().file_name();
    list.push(path.to_str().unwrap().to_string());
  }
  
  return list;
}

pub fn create_dir(path: &str) {
  std::fs::create_dir_all(path).unwrap();
}

pub fn file_exists(path: &str) -> bool {
  return Path::new(path).exists();
}

pub fn create_file_stream(path: &str) -> File {
  return File::create(path).unwrap();
}

pub fn open_file(path: &str) -> File {
  return File::open(path).unwrap()
}

pub fn get_file_size(path: &str) -> io::Result<u64> {
  //let path = Path::new(path);
  // let metadata = path.symlink_metadata()?;

  //let realsize:u64 = path.size_on_disk_fast(&metadata)?;
  let metadata = metadata(path)?;
  let realsize = metadata.len();
  Ok(realsize)
}

// pub fn save_file(path: &str, content: &Cursor<Vec<u8>>) {
//   let mut file = std::fs::File::create(path).unwrap();
//   copy(&mut content.clone(), &mut file).unwrap();
// }
