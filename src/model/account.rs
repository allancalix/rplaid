#[cfg(feature = "decimal")]
use rust_decimal::Decimal;

use super::*;

#[derive(Debug, Serialize)]
pub struct GetAccountsRequest<'a, T: AsRef<str>> {
    pub access_token: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GetAccountsRequestFilter<'a, T>>,
}

#[derive(Debug, Serialize)]
pub struct GetAccountsRequestFilter<'a, T: AsRef<str>> {
    pub account_ids: &'a [T],
}

impl<T: AsRef<str> + serde::Serialize> Endpoint for GetAccountsRequest<'_, T> {
    type Response = GetAccountsResponse;

    fn path(&self) -> String {
        "/accounts/get".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAccountsResponse {
    pub accounts: Vec<Account>,
    pub item: Item,
    pub request_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub account_id: String,
    pub balances: Balance,
    pub mask: Option<String>,
    pub name: String,
    pub official_name: Option<String>,
    /// One of investment | credit | depository | loan | brokerage | other.
    pub r#type: AccountType,
    pub subtype: Option<String>,
    // This field is listed on the documentation for this type as non-nullable
    // but doesn't appear to be returned in payloads.
    // https://plaid.com/docs/api/accounts/#accounts-get-response-verification-status_accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
pub enum AccountType {
    #[serde(rename = "investment")]
    Investment,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "depository")]
    Depository,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    #[cfg(not(feature = "decimal"))]
    pub available: Option<f64>,
    #[cfg(not(feature = "decimal"))]
    pub current: Option<f64>,
    #[cfg(feature = "decimal")]
    pub available: Option<Decimal>,
    #[cfg(feature = "decimal")]
    pub current: Option<Decimal>,
    pub iso_currency_code: Option<String>,
    #[cfg(feature = "decimal")]
    pub limit: Option<Decimal>,
    #[cfg(not(feature = "decimal"))]
    pub limit: Option<f64>,
    pub unofficial_currency_code: Option<String>,
}
