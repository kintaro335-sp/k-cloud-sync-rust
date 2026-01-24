/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use reqwest::{Client, StatusCode, multipart};
use std::{os::unix::fs::FileExt};
use std::time::Duration;
use thiserror::Error;
use std::fs;
use std::io::Write;
use futures_util::StreamExt;
use url::Url;
pub use crate::core::objects;
use tokio::{
    io::{BufReader, AsyncReadExt},
};

const CHUNK_SIZE: u64 = 2097153;

/// Errores de tu capa HTTP/cliente.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("error de red: {0}")]
    Network(#[from] reqwest::Error),

    #[error("error de JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL inválida: {0}")]
    Url(#[from] url::ParseError),

    #[error("IO Error")]
    IOError(#[from] std::io::Error),

    #[error("status HTTP inesperado {status}: {snippet}")]
    HttpStatus {
        status: StatusCode,
        snippet: String,
    },
}

/// Cliente de API con configuración (timeout, headers, etc.).
pub struct ApiClient {
    base: Url,
    api_key: String,
    http: Client,
}

impl ApiClient {
    pub fn new(base_url: &str, api_key: &str) -> Result<Self, ApiError> {
        let http = Client::builder()
            .timeout(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(5))
            // .user_agent("mi-app/0.1") // opcional
            .build()?;
        let api_key = api_key;

        Ok(Self {
            base: Url::parse(base_url)?,
            api_key: api_key.to_string(),
            http,
        })
    }


    
    pub async fn auth(&self) -> Result<objects::User, ApiError> {
      let mut url = self.base.clone();
      url.path_segments_mut()
          .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
          .extend(&["auth"]);

      let resp = self.http.get(url.to_string()+"?t="+&self.api_key).send().await?;
      let status = resp.status();
      let body = resp.text().await?;

      if !status.is_success() {
        let snippet = body.chars().take(200).collect::<String>();
        return Err(ApiError::HttpStatus { status, snippet });
      }

      // Aquí usamos serde_json manualmente
      let user: objects::User = serde_json::from_str(&body)?;
      Ok(user)
    }

    pub async fn get_api_scopes(&self) -> Result<objects::ScopesResp, ApiError> {
      let mut url = self.base.clone();
      url.path_segments_mut()
          .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
          .extend(&["auth", "scopes"]);

      let resp = self.http.get(url.to_string()+"?t="+&self.api_key).send().await?;
      let status = resp.status();
      let body = resp.text().await?;

      if !status.is_success() {
        let snippet = body.chars().take(200).collect::<String>();
        return Err(ApiError::HttpStatus { status, snippet });
      }

      // Aquí usamos serde_json manualmente
      let scopes: objects::ScopesResp = serde_json::from_str(&body)?;
      Ok(scopes)
    }

    pub async fn get_file_properties(&self, path: &str) -> Result<objects::FileProperties, ApiError> {
      let mut url = self.base.clone();
      url.path_segments_mut()
          .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
          .extend(&["files", "properties", path]);
      let resp = self.http.get(url.to_string()+"?t="+&self.api_key).send().await?;
      let status = resp.status();
      let body = resp.text().await?;

      if !status.is_success() {
        let snippet = body.chars().take(200).collect::<String>();
        return Err(ApiError::HttpStatus { status, snippet });
      }

      // Aquí usamos serde_json manualmente
      let properties: objects::FileProperties = serde_json::from_str(&body)?;
      Ok(properties) 
    }

    pub async fn get_files_list(&self, path: &str) -> Result<objects::FileList, ApiError> {
      let mut url = self.base.clone();
      url.path_segments_mut()
          .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
          .extend(&["files", "list", path]);
      let resp = self.http.get(url.to_string()+"?t="+&self.api_key).send().await?;
      let status = resp.status();
      let body = resp.text().await?;

      if !status.is_success() {
        let snippet = body.chars().take(200).collect::<String>();
        return Err(ApiError::HttpStatus { status, snippet });
      }

      // Aquí usamos serde_json manualmente
      let files: objects::FileList = serde_json::from_str(&body)?;
      Ok(files) 
    }

    pub async fn get_file(&self, path: &str, mut file: &fs::File) -> Result<&str, ApiError> {
      let mut url = self.base.clone();
      url.path_segments_mut()
          .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
          .extend(&["files", "list", path]);
      let resp= self.http.get(url.to_string()+"?t="+&self.api_key).send().await.unwrap();
      
      // status
      let status = resp.status();
      if !status.is_success() {
        let body = resp.text().await.unwrap();
        let snippet = body.chars().take(200).collect::<String>();
        return Err(ApiError::HttpStatus { status, snippet });
      }
      
      let mut stream = resp.bytes_stream();
      
      let mut position: u64 = 0;

      while let Some(item) = stream.next().await {
        let chunk = item.unwrap();
        file.write_at(&chunk, position).unwrap();
        position += chunk.len() as u64;
      }
      file.flush().unwrap();
      Ok("okay")
    }

  pub async fn exists_file(&self, path: &str) -> Result<objects::ExistFile, ApiError> {
    let mut url = self.base.clone();
    url.path_segments_mut()
        .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
        .extend(&["files", "exists", path]);
    let resp= self.http.get(url.to_string()+"?t="+&self.api_key).send().await.unwrap();

    // status
    let status = resp.status();
    if !status.is_success() {
      let body = resp.text().await.unwrap();
      let snippet = body.chars().take(200).collect::<String>();
      return Err(ApiError::HttpStatus { status, snippet });
    }
    let body = resp.text().await.unwrap();

    let files: objects::ExistFile = serde_json::from_str(&body)?;
    Ok(files)
  }
  
  pub async fn create_folder(&self, path: &str) -> Result<&str, ApiError> {
    let mut url = self.base.clone();
    url.path_segments_mut()
        .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
        .extend(&["files", "folder", path]);
    let resp= self.http.post(url.to_string()+"?t="+&self.api_key).send().await.unwrap(); 

    // status
    let status = resp.status();
    if !status.is_success() {
      let body = resp.text().await.unwrap();
      let snippet = body.chars().take(200).collect::<String>();
      return Err(ApiError::HttpStatus { status, snippet });
    }

    Ok("okay")
  }

  pub async fn initialize_file(&self, path: &str, size: u64) -> Result<&str, ApiError> {
    let mut url = self.base.clone();
    url.path_segments_mut()
        .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
        .extend(&["files", "initialize", path]);

    let size_body = objects::SizeBody {
      size: size
    };
    let resp = self.http.post(url.to_string()+"?t="+&self.api_key).json(&size_body).send().await.unwrap();  

    // status
    let status = resp.status();
    if !status.is_success() {
      let body = resp.text().await.unwrap();
      let snippet = body.chars().take(200).collect::<String>();
      return Err(ApiError::HttpStatus { status, snippet });
    }

    Ok("okay")
  }

  pub async fn upload_file_chunks(&self, remote_path: &str, path_local: &String, size: u64) -> Result<&str, ApiError> {
    let mut url = self.base.clone();
    url.path_segments_mut()
        .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?
        .extend(&["files", "write", remote_path]);
    let file = tokio::fs::File::open(&path_local).await?;
    let mut reader = BufReader::with_capacity(CHUNK_SIZE as usize, file);
    let mut offset: u64 = 0;

    loop {
      let position_str: String = format!("{}", offset);

      let mut buffer = vec![0u8; CHUNK_SIZE as usize];
      if size > CHUNK_SIZE && size - offset < CHUNK_SIZE {
        buffer = vec![0u8; (size - offset) as usize];
      } else if size < CHUNK_SIZE {
        buffer = vec![0u8; size as usize];
      }

      let bytes_read = reader.read(&mut buffer).await?;

      if bytes_read == 0 {
          break; // EOF
      }

      let part = multipart::Part::stream(buffer)
        .file_name("file")
        .mime_str("application/octet-stream").unwrap();

      let form = multipart::Form::new().part("file", part);
      
      let resp= self.http.post(url.to_string()+"?t="+&self.api_key+"&pos="+&position_str).multipart(form).send().await.unwrap();  

      offset += bytes_read as u64;
      // status
      let status = resp.status();
      if !status.is_success() {
        let body = resp.text().await.unwrap();
        let snippet = body.chars().take(200).collect::<String>();
        return Err(ApiError::HttpStatus { status, snippet });
      }

    }

    Ok("okay")
  }


}
