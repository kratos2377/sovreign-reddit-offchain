use axum::http::{StatusCode};
use axum::response::{IntoResponse , Response};
use axum::Json;
use serde::Serialize;
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;


pub struct ErrorPayloadResponse {
	pub result: SuccessResponse,
	pub error_message: String,
}

pub struct SuccessResponse {
	pub success: bool
}

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
SomeErrorOccurred,
	MissingParams,
	UnexpectedError,
	AuthFailNoAuthTokenCookie,
	AuthFailTokenWrongFormat,
	AuthFailCtxNotInRequestExt,
	ErrorWhileFetchingUserFeed,
	RedisUpdateFailed,
	
	DBInsertError,
	DBFetchError,
	TicketDeleteFailIdNotFound { id: u64 },
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
	//	println!("->> {:<12} - {self:?}", "INTO_RES");
		let (status_code , error_message) = self.client_status_and_error();
		let json_body_string = Json(json!({
			"result": {
				"success": false
			},

			"error_message": format!("{:?}", error_message)
	
		}));

	
		let mut response = Response::builder().status(status_code).body(json_body_string.into_response().into_body()).unwrap();
		
		response.extensions_mut().insert(self);

		response
	}
}

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		#[allow(unreachable_patterns)]
		match self {
			Self::DBFetchError => (StatusCode::BAD_REQUEST , ClientError::DB_FETCH_ERROR), 
			Self::RedisUpdateFailed => (StatusCode::BAD_REQUEST , ClientError::REDIS_UPDATE_FAILED),
			Self::SomeErrorOccurred => (StatusCode::BAD_REQUEST , ClientError::SOME_ERROR_OCCURRED),
			
			Self::MissingParams => (StatusCode::BAD_REQUEST , ClientError::MISSING_PARAMS),

			Self::UnexpectedError => (StatusCode::BAD_REQUEST , ClientError::UNEXPECTED_ERROR),

			Self::DBInsertError => (StatusCode::BAD_REQUEST , ClientError::DB_INSERT_ERROR),
			Self::ErrorWhileFetchingUserFeed => (StatusCode::BAD_REQUEST , ClientError::ERROR_WHILE_FETCHING_USER_FEED),
			// -- Auth.
			Self::AuthFailNoAuthTokenCookie
			| Self::AuthFailTokenWrongFormat
			| Self::AuthFailCtxNotInRequestExt => {
				(StatusCode::FORBIDDEN, ClientError::NO_AUTH)
			}

			// -- Model.
			Self::TicketDeleteFailIdNotFound { .. } => {
				(StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
			}

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	MISSING_PARAMS,
	SOME_ERROR_OCCURRED,
	REDIS_UPDATE_FAILED,
	UNEXPECTED_ERROR,
	ERROR_WHILE_FETCHING_USER_FEED,
	DB_INSERT_ERROR,
	NO_AUTH,
	INVALID_PARAMS,
	SERVICE_ERROR,
	INVALID_EMAIL_USER_KEY,
	DB_FETCH_ERROR
}