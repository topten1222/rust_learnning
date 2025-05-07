use std::io::Write;
use std::path::Path;

use axum::http::StatusCode;
use axum::{
    extract::{Multipart, State}, response::IntoResponse
};
use axum::Json;
use crate::{db::DbPool, models::{api_response::ApiResponse, contact::{Contact, NewContact}}};
use crate::repositories::{contact::ContactRepositoryTrait, contact_repository::ContactRepository};

pub async fn create_contact(
    State(pool): State<DbPool>,
    multipart: Multipart
) -> impl IntoResponse {
    let mut contact = Contact {
        id: 0,
        title: String::new(),
        body: String::new(),
        files: Some(String::new()), // ใช้ Some ให้รองรับไฟล์อัปโหลด
    };

    let res = upload_file(multipart, &mut contact).await;

    if let Err((status_code, message)) = res {
        let response = ApiResponse {
            status: status_code.as_u16() as u128,
            message,
            data: None,
        };
       return (status_code, Json(response)); // Return error response
    }

    // Handle DB interaction if file upload succeeds
    let mut conn = pool.get().expect("Failed to get DB connection");
    let mut repo = ContactRepository::new(&mut conn);
    let new_contact = NewContact {
        id: None,
        title: contact.title,
        body: contact.body,
        files: contact.files,
    };

    match repo.create(new_contact) {
        Ok(contact) => {
            let response = ApiResponse {
                status: StatusCode::CREATED.as_u16() as u128,
                message: "Contact created".to_string(),
                data: Some(contact),
            };
            return (StatusCode::CREATED, Json(response));

        }
        Err(_) => {
            let response = ApiResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as u128,
                message: "Failed to create contact".to_string(),
                data: None,
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    }
}

pub async fn upload_file(
    mut multipart: Multipart, 
    contact: &mut Contact
) -> Result<(), (StatusCode, String)> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();
        
        if name == "file" {
            if field.file_name().unwrap().is_empty() {
                continue; // ถ้าไฟล์ไม่มีชื่อก็ข้ามไป
            }

            let file_name = field.file_name().unwrap_or_default();
            let ext = Path::new(&file_name).extension().ok_or("File Format Invalid");
            
            if !ext.is_ok() {
                return Err((StatusCode::BAD_REQUEST, "Extension Invalid".to_string()));
            }

            let ext_str = ext.unwrap().to_string_lossy().to_lowercase();
            if ext_str != "jpeg" && ext_str != "jpg" {
                return Err((StatusCode::BAD_REQUEST, "File format is not supported".to_string()));
            }

            let upload_dir = "./uploads/";
            contact.files = Some(format!("{}{}", upload_dir, file_name));

            if !Path::new(upload_dir).exists() {
                std::fs::create_dir_all(upload_dir).map_err(|err| {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                })?;
            }

            let mut file = std::fs::File::create(contact.files.clone().unwrap()).map_err(|err| {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            })?;

            // เขียนไฟล์ลงใน disk
            while let Some(chunk_result) = field.chunk().await.transpose() {
                let chunk = chunk_result.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
                file.write_all(chunk.as_ref()).map_err(|err| {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                })?;
            }
        } else if name == "title" {
            contact.title = field.text().await.map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
            if contact.title.is_empty() {
                return Err((StatusCode::BAD_REQUEST, "Title is required".to_string()));
            }
        } else if name == "body" {
            contact.body = field.text().await.map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
            if contact.body.is_empty() {
                return Err((StatusCode::BAD_REQUEST, "Body is required".to_string()));
            }
        } else {
            return Err((StatusCode::BAD_REQUEST, format!("Unknown field: {}", name)));
        }
    }

    Ok(())
}