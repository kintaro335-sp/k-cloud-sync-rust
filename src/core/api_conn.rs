use reqwest::{Client, StatusCode};
use std::os::unix::fs::FileExt;
use std::time::Duration;
use thiserror::Error;
use std::fs::File;
use std::io::Write;
use futures_util::StreamExt;
use url::Url;
mod objects {
  pub use crate::core::objects::{User, FileList, FileProperties};
}


/// Errores de tu capa HTTP/cliente.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("error de red: {0}")]
    Network(#[from] reqwest::Error),

    #[error("error de JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL inválida: {0}")]
    Url(#[from] url::ParseError),

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
            .timeout(Duration::from_secs(10))
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

    pub async fn get_file(&self, path: &str, mut file: &File) -> Result<&str, ApiError> {
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

    

}
