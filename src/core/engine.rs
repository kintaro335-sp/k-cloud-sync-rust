/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */

use std::{println};
use crate::core::{file_conn::{self}, utils::{self}};
use async_recursion::async_recursion;
use tokio::task;
pub use crate::core::objects;
pub use crate::core::api_conn;
pub use crate::core::multi_thr;

// code for single thread

#[async_recursion]
async fn get_files(dirs: &objects::Dirsync, api_client: &api_conn::ApiClient, virtual_path: &str) -> Result<(), Box<dyn std::error::Error>> {
  let local_path = &dirs.local_path;
  let remote_path = &dirs.remote_path;

  if !file_conn::file_exists(&local_path) && virtual_path == "" {
    file_conn::create_dir(&local_path);
    println!("dir created {}",local_path);
  }

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
      } else {
        println!("found {}",utils::create_path(virtual_path, &file.name));
      }
      // println!("{} -> {}\n", file_virtual_path_server, file_virtual_path_local);
    }
  }

  Ok(())
}

async fn upload_file(api_client: &api_conn::ApiClient, local_path: &String, remote_path: &String, size: u64, virtual_path: &String) {
  if size < 104857600 {
    let _ = api_client.upload_small_file(remote_path, local_path).await.unwrap();

    return ()
  }

  let initialize_result = api_client.initialize_file(remote_path, size).await;
  match initialize_result {
      Ok(_) => {
        let _ = api_client.upload_file_chunks(remote_path, local_path, size, virtual_path).await.unwrap();
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
    let virtual_path_file = &utils::create_path(virtual_path, &file);
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
        println!("uploading {}",virtual_path_file);
        let _ = upload_file(api_client, &local_path_file, &remote_path_file, file_size, virtual_path_file).await;
        println!("uploaded  {}",virtual_path_file);
      } else {
        println!("found     {}",virtual_path_file);
      }
    }
  }
  
  Ok(())
}

// end code for single thread

// code for multi thread

#[async_recursion]
async fn get_tasks_files(dirs: &objects::Dirsync, api_client: &api_conn::ApiClient, virtual_path: &str, mut tasks_files: Vec<objects::FileObj>) -> Result<Vec<objects::FileObj>, Box<dyn std::error::Error>>{
  let action = &dirs.sync_mode;
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

  if action == "send" || action == "bidirectional" {
    let files_local_list = file_conn::file_list(&virtual_local_path);
    for file in files_local_list {
      let virtual_path_file = &utils::create_path(virtual_path, &file);
      let local_path_file = &utils::create_path(&virtual_local_path, &file);
      let remote_path_file = &utils::create_path(&virtual_remote_path, &file);

      let is_dir = file_conn::is_dir(local_path_file).unwrap();
      let file_size = file_conn::get_file_size(local_path_file).unwrap();
      let file_remote_exists = api_client.exists_file(&remote_path_file).await.unwrap();

      if is_dir {
        if !file_remote_exists.exists {
          tasks_files.push(objects::FileObj { 
            action: "send".to_string(),
            remote_path: remote_path_file.clone(),
            local_path: local_path_file.clone(),
            size: 1024,
            r#type: "folder".to_string()
          });
        }
        let mut sub_dir_tasks = get_tasks_files(dirs, api_client, &utils::create_path(virtual_path, &virtual_path_file), Vec::new()).await.unwrap();
        tasks_files.append(&mut sub_dir_tasks);
      } else {
        tasks_files.push(objects::FileObj {
          action: "send".to_string(),
          remote_path: remote_path_file.clone(),
          local_path: local_path_file.clone(),
          size: file_size.clone() as usize,
          r#type: "file".to_string()
        });
      }

    }
  }

  println!("{}", virtual_remote_path);

  if action == "get"  || action == "bidirectional" {
    let files_remote_list = api_client.get_files_list(&virtual_remote_path).await.unwrap();
    for file in files_remote_list.list.iter() {
      let file_virtual_path_server = utils::create_path(&virtual_remote_path, &file.name);
      let file_virtual_path_local = utils::create_path(&virtual_local_path, &file.name);
      
      let file_local_exists = file_conn::file_exists(&file_virtual_path_local);
      
      if file.r#type == "folder" {
        if !file_local_exists {
          tasks_files.push(objects::FileObj {
            action: "get".to_string(),
            remote_path: file_virtual_path_server.clone(),
            local_path: file_virtual_path_local.clone(),
            size: file.size.clone() as usize,
            r#type: file.r#type.clone()
          });
        } else {
          let mut sub_dir_tasks = get_tasks_files(dirs, api_client, &utils::create_path(virtual_path, &file.name), Vec::new()).await.unwrap();
          tasks_files.append(&mut sub_dir_tasks);
        }
      } else {
        if !file_conn::file_exists(&file_virtual_path_local) {
          tasks_files.push(objects::FileObj {
            action: "get".to_string(),
            remote_path: file_virtual_path_server.clone(),
            local_path: file_virtual_path_local.clone(),
            size: file.size.clone() as usize,
            r#type: file.r#type.clone()
          });
        }
      }
    }
  }

  Ok(tasks_files)
}

async fn get_plan(dirs: &objects::Dirsync, api_client: &api_conn::ApiClient) -> Vec<objects::FileObj> {
  return get_tasks_files(dirs, api_client, &"".to_string(), Vec::new()).await.unwrap();
}


async fn sync_file(file_info: &objects::FileObj, api_client: &api_conn::ApiClient) -> Result<(), Box<dyn std::error::Error>> {

  if file_info.action == "send" {
    let file_exists = api_client.exists_file(&file_info.remote_path).await.unwrap();
    match file_info.r#type.as_str() {
        "file" => {
          if !file_exists.exists {
            println!("uploading file  {}", file_info.remote_path);
            upload_file(api_client, &file_info.local_path, &file_info.remote_path, file_info.size as u64, &file_info.local_path).await;
            println!("file uploaded   {}", file_info.remote_path);
          }
        },
        "folder" => {
          if !file_exists.exists {
            println!("creating dir    {}", file_info.remote_path);
            api_client.create_folder(&file_info.remote_path).await.unwrap();
            println!("dir created     {}", file_info.remote_path);
          }
        },
        _ => {}
    }
  }
  if file_info.action == "get" {
    let file_exists = file_conn::file_exists(file_info.local_path.as_str());
    match file_info.r#type.as_str() {
        "file" => {
          if !file_exists {
            println!("file downloading {}", file_info.local_path);
            let mut file_local = file_conn::create_file_stream(&file_info.local_path);
            let _ = api_client.get_file(&file_info.remote_path, &mut file_local).await;
            println!("file downloaded  {}", file_info.local_path);
          }
        },
        "folder" => {
          if !file_exists {
            println!("creating dir     {}", file_info.local_path);
            file_conn::create_dir(&file_info.local_path);
            println!("dir created      {}", file_info.local_path);
          }
        },
        _ => {}
    }
  }

  Ok(())
} 

// end code for multi thread

// main function

pub async fn sync_files(dir: &objects::Dirsync, api_client: &api_conn::ApiClient, jobs: &u16) -> Result<String, Box<dyn std::error::Error>> {
  let sync_mode = &dir.sync_mode;
  let remote_path = &dir.remote_path;

  let file_properties = api_client.get_file_properties(&remote_path).await.unwrap();

  if file_properties.r#type == "file" {
    return Ok(String::from("files cannot be synced"));
  }

  if *jobs == 1 {
    if sync_mode == "get" || sync_mode == "bidirectional" {
      let _ = get_files(dir, api_client, "").await;
    }
    if sync_mode == "send" || sync_mode == "bidirectional" {
      let _ = send_files(dir, api_client, "").await;
    }
  } else {
    let tasks_files: Vec<objects::FileObj>;

    tasks_files = get_plan(&dir, api_client).await;
    
    let tail_tasks = multi_thr::TailTasks::new(tasks_files);

    let mut handles= Vec::new();

    for _ in 1..*jobs {
      let shared_tail = tail_tasks.clone();
      let api_conn_clone = api_client.clone();

      let handle = task::spawn_blocking(async move || {
        while let Some(file) = shared_tail.take_task() {
            sync_file(&file, &api_conn_clone).await.unwrap();
        }
      });

      handles.push(handle);
    }

    for handle in handles {
      let _ = handle.await.unwrap().await;
    }

  }

  Ok(String::from("sync finished"))  
}
