use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::http::Status;
use rocket::serde::json;

pub enum CustomResponse {
    Text(Status, String),
    Json(Status, json::Value),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for CustomResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        match self {
            CustomResponse::Text(status, content) => {
                let body = content.clone();
                Response::build()
                    .status(status)
                    .sized_body(body.as_bytes().len(), std::io::Cursor::new(content))
                    .ok()
            }
            CustomResponse::Json(status, json_value) => {
                let body =serde_json::to_string(&json_value).unwrap();
                Response::build()
                    .status(status)
                    .header(rocket::http::ContentType::JSON)
                    .sized_body(body.as_bytes().len(), std::io::Cursor::new(body))
                    .ok()
            }
        }
    }
}
