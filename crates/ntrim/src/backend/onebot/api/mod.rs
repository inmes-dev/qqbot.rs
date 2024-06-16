pub(crate) mod account;
pub(crate) mod message;

use actix_web::http::StatusCode;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OnebotRequest {
    pub action: String,
    pub params: serde_json::Value,
    pub echo: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnebotResult {
    pub status: String,
    pub retcode: i32,
    pub data: serde_json::Value,
    pub msg: String,
    pub wording: String,
    pub echo: serde_json::Value,
}

impl OnebotResult {
    pub fn success(data: serde_json::Value, echo: serde_json::Value) -> Self {
        OnebotResult {
            status: "ok".to_string(),
            retcode: 0,
            data,
            msg: "".to_string(),
            wording: "".to_string(),
            echo,
        }
    }

    pub fn failed(msg: String, wording: String, echo: serde_json::Value) -> Self {
        OnebotResult {
            status: "failed".to_string(),
            retcode: -1,
            data: serde_json::Value::Null,
            msg,
            wording,
            echo,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum OnebotError {
    #[error("an unspecified internal error occurred: {0}")]
    InternalError(String),
    #[error("an logic error occurred: {0}")]
    LogicError(String),
    #[error("unsupported Content-Type error occurred: {0}")]
    UnsupportedContentTypeError(String),
    #[error("illegal input: {0}")]
    IllegalInputError(String),
}

impl OnebotError {
    #[inline]
    fn internal(str: &str) -> Self {
        Self::InternalError(str.to_string())
    }

    #[inline]
    fn logic(str: &str) -> Self {
        Self::LogicError(str.to_string())
    }

    #[inline]
    fn content_type_error(str: &str) -> Self {
        Self::UnsupportedContentTypeError(str.to_string())
    }

    #[inline]
    fn illegal_input(str: &str) -> Self {
        Self::IllegalInputError(str.to_string())
    }
}

impl actix_web::ResponseError for OnebotError {

    fn status_code(&self) -> StatusCode {
        match &self {
            Self::InternalError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            OnebotError::LogicError(..) => StatusCode::BAD_REQUEST,
            OnebotError::UnsupportedContentTypeError(..) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            OnebotError::IllegalInputError(..) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let data = serde_json::to_string(
            &OnebotResult::failed(
                self.to_string(),
                "".to_string(),
                serde_json::Value::Null,
            )
        ).map_err(|e| {
            error!("Failed to serialize error response: {}", e);
        }).unwrap_or("UNKNOWN FAILED".to_string());
        actix_web::HttpResponse::build(self.status_code()).body(data)
    }
}

#[macro_export]
macro_rules! init_route {
    ($route:expr, $struct_name:ident, $handler:expr) => {
        use crate::backend::onebot::api::OnebotError;
        use tokio_stream::StreamExt;
        use actix_web::{Error, Responder, web};

        pub fn register(cfg: &mut web::ServiceConfig) {
        type Params = $struct_name;

        async fn handle_get(req: actix_web::HttpRequest) -> actix_web::Result<String> {
            let data = req.app_data::<web::Data<Arc<Bot>>>().unwrap();
            let bot = data.get_ref();
            let params = req.query_string();
            let params = match serde_urlencoded::from_str::<Params>(params) {
                Ok(params) => params,
                Err(e) => {
                    let oe = OnebotError::LogicError(format!("query params failed: {}", e));
                    return Err(Error::from(oe));
                }
            };
            let resp = $handler(bot, params).await?;
            let resp = serde_json::to_value(resp).unwrap();
            let data = serde_json::to_string(
                &crate::backend::onebot::api::OnebotResult::success(resp, serde_json::Value::Null)
            ).unwrap();
            Ok(data)
        }

        async fn handle_post(req: actix_web::HttpRequest, payload: web::Payload) -> impl Responder {
            let content_type = req.headers().get("Content-Type").and_then(|ct| ct.to_str().ok());
            match content_type {
                Some("application/json") => handle_post_json(req, payload).await,
                Some("application/x-www-form-urlencoded") => handle_post_urlencoded(req, payload).await,
                _ => {
                    let oe = content_type.unwrap_or("").to_string();
                    Err(Error::from(OnebotError::UnsupportedContentTypeError(oe)))
                },
            }
        }

        async fn handle_post_json(req: actix_web::HttpRequest, mut payload: web::Payload) -> actix_web::Result<String> {
            let mut body = web::BytesMut::new();
            while let Some(chunk) = payload.next().await {
                body.extend_from_slice(&chunk.unwrap());
            }

            let params: Params = match serde_json::from_slice(&body) {
                Ok(val) => val,
                Err(e) => {
                    return Err(Error::from(OnebotError::LogicError(format!("query params failed: {}", e))));
                }
            };

            let data = req.app_data::<web::Data<Arc<Bot>>>().unwrap();
            let bot = data.get_ref();

            let resp = $handler(bot, params).await?;
            let resp = serde_json::to_value(resp).unwrap();
            let data = serde_json::to_string(
                &crate::backend::onebot::api::OnebotResult::success(resp, serde_json::Value::Null)
            ).unwrap();
            Ok(data)
        }

        async fn handle_post_urlencoded(req: actix_web::HttpRequest, mut payload: web::Payload) -> actix_web::Result<String> {
            let mut body = web::BytesMut::new();
            while let Some(chunk) = payload.next().await {
                body.extend_from_slice(&chunk.unwrap());
            }

            let data = req.app_data::<web::Data<Arc<Bot>>>().unwrap();
            let bot = data.get_ref();

            let params = match serde_urlencoded::from_bytes::<Params>(body.as_ref()) {
                Ok(val) => val,
                Err(e) => {
                    let oe = OnebotError::LogicError(format!("query params failed: {}", e));
                    return Err(Error::from(oe));
                }
            };

            let resp = $handler(bot, params).await?;
            let resp = serde_json::to_value(resp).unwrap();
            let data = serde_json::to_string(
                &crate::backend::onebot::api::OnebotResult::success(resp, serde_json::Value::Null)
            ).unwrap();
            Ok(data)
        }

        cfg.service(web::scope($route)
            .route("", web::get().to(handle_get))
            .route("", web::post().to(handle_post))
        );
    }
    };
}