use std::io::Write;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::extract::Multipart;
use std::path::Path;
use axum::Json;

use crate::models::api_response::ApiResponse;
use crate::models::contact::Contact;

pub async fn create_contact(multipart: Multipart) -> impl IntoResponse {
    let contact = Contact {
        title: String::new(),
        body: String::new(),
        file: String::new(),
    };
    let reponse = upload_file(multipart, contact).await.map_err(|err| {
        let response = ApiResponse {
            status: err.0.as_u16() as u128,
            message: err.1,
            data: None::<()>,
        };
        (err.0, Json(response))
    });
    match reponse {
        Result::Ok(_) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16() as u128,
                message: "OK".to_string(),
                data: None::<()>,
            };
            (StatusCode::OK, Json(response))
        }
        Err(err) => err,
    }
}

pub async fn upload_file(mut multipart: Multipart, mut contact: Contact) -> Result<(), (StatusCode, String)> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();
            if let Some(value) = field.file_name() {
                let upload_dir =  "./uploads/";
                contact.file = format!("{}{}", upload_dir, value);
                if !Path::new(upload_dir).exists() {
                    std::fs::create_dir_all(upload_dir).map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
                }
                let mut file = std::fs::File::create(contact.file).map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
                
                while let Some(chunk_result) = field.chunk().await.transpose() {
                    let chunk = chunk_result.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
                    file.write_all(chunk.as_ref())
                        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
                }
            } else if name == "title" {
                contact.title = field.text().await.map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
            } else if name == "body" {
                contact.body = field.text().await.map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
            }
    }
    Result::Ok(())
}