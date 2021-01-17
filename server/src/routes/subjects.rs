use actix_web::{delete, get, patch, post, web, Error, HttpResponse, Result};
use http::StatusCode;

use datamodel::*;
extern crate back_end;
use back_end::*;

#[post("/subject")]
pub async fn create_subject(
    new_subject: web::Json<ListPayload>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let new_title_result = match new_subject.title.clone() {
        None => Err(HttpErrors::BadClientData),
        Some(title) => Ok(title),
    };
    let new_subject = match new_subject.clone(){
        None => Err(HttpErrors::BadClientData),
        Some(subject) => Ok(NewSubject {title:subject.title, description:subject.description})
    };
    match new_subject {
        Ok(subject) => {
            let created_subject = web::block(move || subject.save(&conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::build(StatusCode::NOT_FOUND).finish()
            })?;
            Ok(HttpResponse::build(StatusCode::CREATED).json(created_subject))
        }
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    }
}