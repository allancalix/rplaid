use super::*;

#[derive(Debug, Serialize)]
pub struct GetItemRequest<T: AsRef<str>> {
    pub access_token: T,
}

impl<T: AsRef<str> + serde::Serialize> Endpoint for GetItemRequest<T> {
    type Response = GetItemResponse;

    fn path(&self) -> String {
        "/item/get".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetItemResponse {
    pub item: Item,
    pub status: Option<Status>,
    pub request_id: String,
}

#[derive(Debug, Serialize)]
pub struct RemoveItemRequest<T: AsRef<str>> {
    pub access_token: T,
}

impl<T: AsRef<str> + serde::Serialize> Endpoint for RemoveItemRequest<T> {
    type Response = RemoveItemResponse;

    fn path(&self) -> String {
        "/item/remove".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveItemResponse {
    pub request_id: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateItemWebhookRequest<T: AsRef<str>> {
    pub access_token: T,
    /// The new url to associate with the item.
    pub webhook: T,
}

impl<T: AsRef<str> + serde::Serialize> Endpoint for UpdateItemWebhookRequest<T> {
    type Response = UpdateItemWebhookResponse;

    fn path(&self) -> String {
        "/item/webhook/update".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateItemWebhookResponse {
    pub item: Item,
    pub request_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub item_id: String,
    pub institution_id: Option<String>,
    pub webhook: Option<String>,
    pub error: Option<ErrorResponse>,
    pub available_products: Vec<String>,
    pub billed_products: Vec<String>,
    // An RFC 3339 timestamp after which the consent provided by the end user will expire.
    pub consent_expiration_time: Option<String>,
    pub update_type: String,
    pub status: Option<Status>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    pub investments: Option<StatusMessage>,
    pub transactions: Option<StatusMessage>,
    pub last_webhook: Option<WebhookStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusMessage {
    pub last_successful_update: Option<String>,
    pub last_failed_update: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookStatus {
    pub sent_at: Option<String>,
    pub code_sent: Option<String>,
}
