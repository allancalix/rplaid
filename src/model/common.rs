use super::*;

pub(crate) trait Endpoint: HttpSerialize {
    type Response;

    fn path(&self) -> String;

    fn request(&self, domain: &str) -> http_client::Request {
        let mut req = http_client::Request::post(format!("{}{}", domain, self.path()).as_str());
        req.set_body(self.payload());

        req
    }

    fn payload(&self) -> http_types::Body {
        http_types::Body::from_json(&self).unwrap()
    }
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize, Default)]
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
