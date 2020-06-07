use crate::models::response::ResponseBody;
use actix_web::{http::StatusCode, HttpResponse};

pub type ServiceResult<T> = std::result::Result<T, ServiceError>;

pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: impl Into<String>) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message: message.into(),
                data: String::new(),
            },
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}
