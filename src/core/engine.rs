use std::os::unix::fs::MetadataExt;

/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use crate::core::{file_conn, utils::create_path};
use async_recursion::async_recursion;
mod objects {
  pub use crate::core::objects::{Dirsync};
}
mod api_conn {
  pub use crate::core::api_conn::ApiClient;
}

#[async_recursion]
async fn get_files(dirs: &objects::Dirsync, api_client: &api_conn::ApiClient, virtual_path: &str) -> Result<(), Box<dyn std::error::Error>> {
  let local_path = &dirs.local_path;
  let remote_path = &dirs.remote_path;

  let virtual_local_path: String;
  let virtual_remote_path: String;

  if virtual_path.is_empty() {
    virtual_local_path = local_path.clone();
    virtual_remote_path = remote_path.clone();    
  } else {
    virtual_local_path = format!("{}/{}", local_path, virtual_path);
    virtual_remote_path = format!("{}/{}", remote_path, virtual_path);
  }

  let files_server_list = api_client.get_files_list(&virtual_remote_path).await.unwrap();

  for file in files_server_list.list.iter() {
    let file_virtual_path_server = format!("{}/{}", virtual_remote_path, file.name);
    let file_virtual_path_local = format!("{}/{}", virtual_local_path, file.name);
    if file.r#type == "folder" {
      if !file_conn::file_exists(&file_virtual_path_local) {
        file_conn::create_dir(&file_virtual_path_local);
      }
      let _ = get_files(dirs, api_client, &create_path(virtual_path, &file.name)).await;

    } else {
      if !file_conn::file_exists(&file_virtual_path_local) {
        let mut file_local = file_conn::create_file_stream(&file_virtual_path_local);
        let _ = api_client.get_file(&file_virtual_path_server, &mut file_local).await;
        // drop(file_local);
      }
      // println!("{} -> {}\n", file_virtual_path_server, file_virtual_path_local);
    }
  }

  Ok(())
}

async fn upload_file(api_client: &api_conn::ApiClient, local_path: &String, remote_path: &String, size: usize) {
  println!("uploading");
  let _ = api_client.initialize_file(remote_path, size).await.unwrap();
  let _ = api_client.upload_file_chunks(remote_path, local_path, size).await.unwrap();
}

#[async_recursion]
async fn send_files(dirs: &objects::Dirsync, api_client: &api_conn::ApiClient, virtual_path: &str) -> Result<(), Box<dyn std::error::Error>> {
  let local_path = &dirs.local_path;
  let remote_path = &dirs.remote_path;

  let virtual_local_path: String;
  let virtual_remote_path: String;

  if virtual_path.is_empty() {
    virtual_local_path = local_path.clone();
    virtual_remote_path = remote_path.clone();    
  } else {
    virtual_local_path = format!("{}/{}", local_path, virtual_path);
    virtual_remote_path = format!("{}/{}", remote_path, virtual_path);
  }
  
  let files_local_list = file_conn::file_list(&virtual_local_path);

  for file in files_local_list {

    let local_path_file = &create_path(&virtual_local_path, &file);
    let remote_path_file = &create_path(&virtual_remote_path, &file);

    let file_properties = file_conn::open_file(&virtual_local_path).metadata().unwrap();
    let is_dir = file_properties.is_dir();
    
    let exists_file_remote = api_client.exists_file(&remote_path_file).await.unwrap();

    if !is_dir {
      if !exists_file_remote.exists {
        api_client.create_folder(&remote_path_file).await.unwrap();
      }
      let _ = send_files(dirs, api_client, &file).await.unwrap();
    } else {
      if !exists_file_remote.exists {
        let _ = upload_file(api_client, &local_path_file, &remote_path_file, file_properties.size() as usize).await;
      }
    }
  }
  
  Ok(())
}

pub async fn sync_files(dir: &objects::Dirsync, api_client: &api_conn::ApiClient) -> Result<String, Box<dyn std::error::Error>> {
  let sync_mode = &dir.sync_mode;
  let remote_path = &dir.remote_path;

  let file_properties = api_client.get_file_properties(&remote_path).await.unwrap();

  if file_properties.r#type == "file" {
    return Ok(String::from("files cannot be synced"));
  }

  if sync_mode == "get" || sync_mode == "bidirectional" {
    let _ = get_files(dir, api_client, "").await;
  }
  if sync_mode == "send" || sync_mode == "bidirectional" {
    let _ = send_files(dir, api_client, "").await;
  }

  Ok(String::from("sync finished"))  
}
