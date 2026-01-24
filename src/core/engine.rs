/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use crate::core::{file_conn, utils::{self}};
use async_recursion::async_recursion;
pub use crate::core::objects;
pub use crate::core::api_conn;


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
    virtual_local_path = utils::create_path(local_path, virtual_path);
    virtual_remote_path = utils::create_path(remote_path, virtual_path);
  }

  let files_server_list = api_client.get_files_list(&virtual_remote_path).await.unwrap();

  for file in files_server_list.list.iter() {
    let file_virtual_path_server = utils::create_path(&virtual_remote_path, &file.name);
    let file_virtual_path_local = utils::create_path(&virtual_local_path, &file.name);
    if file.r#type == "folder" {
      if !file_conn::file_exists(&file_virtual_path_local) {
        file_conn::create_dir(&file_virtual_path_local);
        println!("dir created {}",utils::create_path(virtual_path, &file.name));
      }
      let _ = get_files(dirs, api_client, &utils::create_path(virtual_path, &file.name)).await;

    } else {
      if !file_conn::file_exists(&file_virtual_path_local) {
        println!("downloading {}",utils::create_path(virtual_path, &file.name));
        let mut file_local = file_conn::create_file_stream(&file_virtual_path_local);
        let _ = api_client.get_file(&file_virtual_path_server, &mut file_local).await;
        println!("downloaded  {}",utils::create_path(virtual_path, &file.name));
        // drop(file_local);
      }
      // println!("{} -> {}\n", file_virtual_path_server, file_virtual_path_local);
    }
  }

  Ok(())
}

async fn upload_file(api_client: &api_conn::ApiClient, local_path: &String, remote_path: &String, size: u64) {
  let initialize_result = api_client.initialize_file(remote_path, size).await;
  match initialize_result {
      Ok(_) => {
        let _ = api_client.upload_file_chunks(remote_path, local_path, size).await.unwrap();
      },
      Err(err) => {
        println!("{}", err);
      }
  }
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
    virtual_local_path = utils::create_path(local_path, virtual_path);
    virtual_remote_path = utils::create_path(remote_path, virtual_path);
  }
  
  let files_local_list = file_conn::file_list(&virtual_local_path);
  let files_remote_list = api_client.get_files_list(&virtual_remote_path).await.unwrap();

  for file in files_local_list {

    let local_path_file = &utils::create_path(&virtual_local_path, &file);
    let remote_path_file = &utils::create_path(&virtual_remote_path, &file);

    let is_dir = file_conn::is_dir(local_path_file).unwrap();
    let file_size = file_conn::get_file_size(local_path_file).unwrap();
    let exists_file_remote = utils::exists_file_remote(&files_remote_list, &file);

    if is_dir {
      if !exists_file_remote {
        api_client.create_folder(&remote_path_file).await.unwrap();
        println!("dir created {}",utils::create_path(virtual_path, &file));
      }
      let _ = send_files(dirs, api_client, &file).await.unwrap();
    } else {
      if !exists_file_remote {
        println!("uploading {}",utils::create_path(virtual_path, &file));
        let _ = upload_file(api_client, &local_path_file, &remote_path_file, file_size).await;
        println!("uploaded  {}",utils::create_path(virtual_path, &file));
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
