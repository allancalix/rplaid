use super::*;

pub(crate) trait Endpoint: serde::Serialize {
    type Response;

    fn path(&self) -> String;

    fn payload(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(thiserror::Error, Debug, Deserialize, Serialize, Eq, PartialEq, Default)]
#[error("request failed with code {error_code:?}: {display_message:?}")]
pub struct ErrorResponse {
    pub display_message: Option<String>,
    pub documentation_url: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub error_type: Option<ErrorType>,
    pub request_id: Option<String>,
    pub status: Option<u32>,
    pub suggested_action: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorType {
    InvalidRequest,
    InvalidResult,
    InvalidInput,
    InstitutionError,
    RateLimitExceeded,
    ApiError,
    ItemError,
    AssetReportError,
    RecaptchaError,
    OauthError,
    PaymentError,
    BankTransferError,
}
