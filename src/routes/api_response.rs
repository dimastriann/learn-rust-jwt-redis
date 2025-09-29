use std::borrow::Cow;
use actix_web::{HttpResponse, error::Error};
use serde::Serialize;
use crate::models::base_model::ApiResponse;

impl <T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }

    pub fn ok_msg<M: Into<Cow<'static, str>>>(data: T, msg: M) -> Self {
        Self {
            success: true,
            data,
            message: Some(msg.into().parse().unwrap())
        }
    }
}

pub fn api_response<T: Serialize>(result: Result<T, impl std::error::Error>) -> Result<HttpResponse, Error> {
    match result {
        Ok(data) => Ok(HttpResponse::Ok().json(ApiResponse::ok(data))),
        Err(err) => Ok(HttpResponse::NotFound().json(ApiResponse {
            success: false,
            data: None::<T>,
            message: Some(err.to_string()),
        })),
    }
}