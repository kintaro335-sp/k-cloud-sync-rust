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

  Ok(String::from("sync finished"))  
}
