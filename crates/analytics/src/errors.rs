use api_models::errors::types::{ApiError, ApiErrorResponse};
use common_utils::errors::{CustomResult, ErrorSwitch};

pub type AnalyticsResult<T> = CustomResult<T, AnalyticsError>;

#[derive(Debug, Clone, serde::Serialize, thiserror::Error)]
pub enum AnalyticsError {
    #[allow(dead_code)]
    #[error("Not implemented: {0}")]
    NotImplemented(&'static str),
    #[error("Unknown Analytics Error")]
    UnknownError,
    #[error("Access Forbidden Analytics Error")]
    AccessForbiddenError,
    #[error("Failed to fetch currency exchange rate")]
    ForexFetchFailed,
}

impl ErrorSwitch<ApiErrorResponse> for AnalyticsError {
    fn switch(&self) -> ApiErrorResponse {
        match self {
            Self::NotImplemented(feature) => ApiErrorResponse::NotImplemented(ApiError::new(
                "IR",
                0,
                format!("{feature} is not implemented."),
                None,
            )),
            Self::UnknownError => ApiErrorResponse::InternalServerError(ApiError::new(
                "HE",
                0,
                "Something went wrong",
                None,
            )),
            Self::AccessForbiddenError => {
                ApiErrorResponse::Unauthorized(ApiError::new("IR", 0, "Access Forbidden", None))
            }
            Self::ForexFetchFailed => ApiErrorResponse::InternalServerError(ApiError::new(
                "HE",
                0,
                "Failed to fetch currency exchange rate",
                None,
            )),
        }
    }
}
