use std::{fs::read_dir, fs::File};
use std::path::Path;
// use std::io::Cursor;
// use std::io::copy;

pub fn file_list(path: &str) -> Vec<String> {
  let mut list : Vec<String> = Vec::new();

  for entry in read_dir(path).unwrap() {
    let path = entry.unwrap().path();
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

pub fn get_file_size(path: &str) -> u64 {
  return std::fs::metadata(path).unwrap().len();
}

// pub fn save_file(path: &str, content: &Cursor<Vec<u8>>) {
//   let mut file = std::fs::File::create(path).unwrap();
//   copy(&mut content.clone(), &mut file).unwrap();
// }
