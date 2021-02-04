use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};

#[post("/import")]
async fn import(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        if let Some(content_type) = field.content_disposition() {
            return match content_type.get_filename() {
                Some(filename) => {
                    let mut file = std::fs::File::create(filename)?;

                    while let Some(chunk) = field.next().await {
                        file.write_all(&chunk?)?;
                    }

                    Ok(HttpResponse::Ok().into())
                }
                _ => Ok(HttpResponse::BadRequest().into()),
            };
        }
    }

    Ok(HttpResponse::BadRequest().into())
}
