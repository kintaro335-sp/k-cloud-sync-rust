use tokio;

mod core {
    pub mod objects;
    pub mod config_file;
    pub mod file_conn;
    pub mod api_conn;
    pub mod engine;
    pub mod utils;
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: core::objects::Configfile = core::config_file::load_config().expect("Unable to load config");

    let base_url = config.base_url;
    let api_key = config.api_key;
    let dirs = config.dirs;
    let mut user_name: String = String::from("");
    let mut user_id: String = String::from("");
    let mut session_id: String = String::from("");
    let mut authenticated:bool = false;

    let api_client: core::api_conn::ApiClient = core::api_conn::ApiClient::new(&base_url, &api_key).expect("Unable to create api client");

    println!("Authenticating...");
    
    match api_client.auth().await {
        Ok(user) => {
          authenticated = true;
          user_name = user.username;
          user_id = user.userId;
          session_id = user.sessionId;
        },
        Err(err) => println!("Error: {}", err),
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
        Ok(_) => println!("Sync finished"),
        Err(err) => println!("Error: {}", err),
      }
    }

    Ok(())
}
