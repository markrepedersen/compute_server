use std::{fs::File, io::Write};

use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};

#[post("/import")]
async fn import(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut is_file_upload = false;

    while let Some(mut field) = payload.try_next().await? {
        if let Some(content_type) = field.content_disposition() {
            if let Some(filename) = content_type.get_filename() {
                let mut file = File::create(filename)?;

                while let Some(chunk) = field.next().await {
                    file.write_all(&chunk?)?;
                }

                is_file_upload = true;
            }
        }
    }

    Ok(match is_file_upload {
        false => HttpResponse::BadRequest().into(),
        true  => HttpResponse::Ok().into(),
    })
}
