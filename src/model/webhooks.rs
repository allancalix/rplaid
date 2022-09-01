use super::*;

#[derive(Debug, Serialize, Copy, Clone)]
pub struct GetWebhookVerificationKeyRequest<T: AsRef<str>> {
    pub key_id: T,
}

impl<T: AsRef<str> + serde::Serialize> Endpoint for GetWebhookVerificationKeyRequest<T> {
    type Response = GetWebhookVerificationKeyResponse;

    fn path(&self) -> String {
        "/webhook_verification_key/get".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetWebhookVerificationKeyResponse {
    // TODO(allancalix): This is obviously not right, but maybe it's worth
    // bringing in a real JWT type to return here? Creating a JWT type to
    // return here doesn't feel like the right answer.
    pub key: std::collections::HashMap<String, String>,
    pub request_id: String,
}
