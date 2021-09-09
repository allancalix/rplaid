use super::*;

#[derive(Debug, Serialize, Default)]
pub struct CreatePublicTokenRequest<'a, T: AsRef<str>> {
    pub institution_id: T,
    pub initial_products: &'a [T],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreatePublicTokenOptions<T>>,
}

#[derive(Debug, Serialize)]
pub struct CreatePublicTokenOptions<T: AsRef<str>> {
    pub webhook: Option<T>,
    /// Default username is "user_good".
    pub override_username: Option<T>,
    /// Default password is "pass_good".
    pub override_password: Option<T>,
    pub transactions: Option<CreatePublicTokenOptionsTransactions<T>>,
}

#[derive(Debug, Serialize)]
pub struct CreatePublicTokenOptionsTransactions<T: AsRef<str>> {
    pub start_date: Option<T>,
    pub end_date: Option<T>,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for CreatePublicTokenRequest<'_, T> {
    type Response = CreatePublicTokenResponse;

    fn path(&self) -> String {
        "/sandbox/public_token/create".into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePublicTokenResponse {
    pub public_token: String,
}

#[derive(Debug, Serialize)]
pub struct ResetLoginRequest<T: AsRef<str>> {
    pub access_token: T,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for ResetLoginRequest<T> {
    type Response = ResetLoginResponse;

    fn path(&self) -> String {
        "/sandbox/item/reset_login".into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetLoginResponse {
    pub reset_login: bool,
}

#[derive(Debug, Serialize)]
pub struct SetVerificationStatusRequest<T: AsRef<str>> {
    pub access_token: T,
    pub account_id: T,
    /// One of automatically_verified or verification_required
    pub verification_status: T,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for SetVerificationStatusRequest<T> {
    type Response = SetVerificationStatusResponse;

    fn path(&self) -> String {
        "/sandbox/item/set_verification_status".into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetVerificationStatusResponse {
    pub reset_login: bool,
}

#[derive(Debug, Serialize)]
pub struct FireWebhookRequest<T: AsRef<str>> {
    pub access_token: T,
    /// One of DEFAULT_UPDATE.
    pub webhook_code: WebhookCode,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
pub enum WebhookCode {
    #[serde(rename = "DEFAULT_UPDATE")]
    DefaultUpdate,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for FireWebhookRequest<T> {
    type Response = FireWebhookResponse;

    fn path(&self) -> String {
        "/sandbox/item/fire_webhook".into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FireWebhookResponse {
    pub webhook_fired: bool,
    pub request_id: String,
}
