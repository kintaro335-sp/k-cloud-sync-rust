/*
 * k-cloud-sync-rust
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */

use tokio;
use std::env;

use crate::core::utils;

pub mod core {
    pub mod objects;
    pub mod config_file;
    pub mod file_conn;
    pub mod api_conn;
    pub mod engine;
    pub mod utils;
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_file: String = String::from("config.json");
    for (i, arg) in env::args().enumerate() {
      if i == 1 {
        println!("{}", arg);
        config_file = arg;
        break;
      }
    }

    let exist_config_file: bool;
    let exist_result = core::config_file::exists_file(&config_file);

    match exist_result {
        Ok(exist_config) => exist_config_file = exist_config,
        Err(error) => {
          println!("error verifying file existence");
          return Ok(())
        }
    }
    if !exist_config_file {
      println!("config file {} does not exists",config_file);
      return Ok(())
    }

    let config: core::objects::Configfile = core::config_file::load_config(&config_file).expect("Unable to load config");

    let base_url = config.base_url;
    let api_key = config.api_key;
    let dirs = config.dirs;
    let user_name: String;
    let user_id: String;
    let session_id: String;
    let authenticated:bool;

    let api_client: core::api_conn::ApiClient = core::api_conn::ApiClient::new(&base_url, &api_key).expect("Unable to create api client");

    println!("Authenticating...");
    
    match api_client.auth().await {
        Ok(user) => {
          authenticated = true;
          user_name = user.username;
          user_id = user.userId;
          session_id = user.sessionId;
        },
        Err(err) => {
          println!("Error: {}", err);
          return Ok(())
        },
    }

    let scopes_api = api_client.get_api_scopes().await.unwrap();
    let has_permissions = utils::is_authorized(&scopes_api);
    if !has_permissions {
      println!("api key has not enough permissions");
      return Ok(())
    }

    if authenticated {
      println!("Authenticated as {}", user_name);
      println!("User ID: {}", user_id);
      println!("Session ID: {}", session_id);
    } else {
      println!("Not authenticated");
      return Ok(());
    }

    println!("Syncing files...");
    
    for dir in dirs.iter() {
      match core::engine::sync_files(&dir, &api_client).await {
        Ok(message) => println!("{}", message),
        Err(err) => println!("Error: {}", err),
      }
    }

    Ok(())
}
