use super::*;

#[derive(Debug, Serialize, Copy, Clone)]
pub struct GetTransactionsRequest<T: AsRef<str>> {
    pub access_token: T,
    /// A string date with the format YYYY-MM-DD. Start date is inclusive.
    pub start_date: T,
    /// A string date with the format YYYY-MM-DD. End date is inclusive.
    pub end_date: T,
    pub options: Option<GetTransactionsOptions<T>>,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for GetTransactionsRequest<T> {
    type Response = GetTransactionsResponse;

    fn path(&self) -> String {
        "/transactions/get".into()
    }
}

#[derive(Debug, Serialize, Copy, Clone)]
pub struct GetTransactionsOptions<T: AsRef<str>> {
    pub account_ids: Option<T>,
    pub count: Option<usize>,
    pub offset: Option<usize>,
    pub include_original_description: Option<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTransactionsResponse {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
    pub total_transactions: usize,
    pub item: Item,
    pub request_id: String,
}

#[derive(Debug, Serialize, Copy, Clone)]
pub struct RefreshTransactionsRequest<T: AsRef<str>> {
    pub access_token: T,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for RefreshTransactionsRequest<T> {
    type Response = RefreshTransactionsResponse;

    fn path(&self) -> String {
        "/transactions/refresh".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTransactionsResponse {
    pub request_id: String,
}

#[derive(Debug, Serialize, Copy, Clone)]
pub struct GetCategoriesRequest {}

impl Endpoint for GetCategoriesRequest {
    type Response = GetCategoriesResponse;

    fn path(&self) -> String {
        "/categories/get".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetCategoriesResponse {
    pub categories: Vec<Category>,
    pub request_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Category {
    category_id: String,
    group: String,
    hierarchy: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transaction {
    /// DEPRECATED: do not depend on this type, it will be deleted in the future.
    pub transaction_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<TransactionLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_meta: Option<PaymentMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_description: Option<String>,
    pub account_id: String,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
    pub date: String,
    pub pending: bool,
    pub transaction_id: String,
    pub payment_channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_datetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransactionLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_number: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaymentMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppd_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_order_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_processor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
