use std::{fs::File, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error(transparent)]
    Disconnect(#[from] io::Error),
}

impl From<MyError> for AppError {
    fn from(error: MyError) -> Self {
        match error {
            MyError::Disconnect(e) => AppError {
                statuscode: StatusCode::InternalServerError,
                error: format!("{}", e),
                message: "Failed to connect file!".to_string(),
            },
        }
    }
}

#[derive(Debug)]
pub enum StatusCode {
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
}

#[derive(Debug)]
pub struct AppError {
    statuscode: StatusCode,
    error: String,
    message: String,
}

// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         Response {
//             statuscode: self.statuscode,
//             error: self.error,
//             message: self.message,
//         }
//     }
// }

fn main() -> Result<(), AppError> {
    let file = File::open("data.csv").map_err(MyError::from)?;
    Ok(())
}
