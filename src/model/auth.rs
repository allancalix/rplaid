use super::*;

#[derive(Debug, Serialize)]
pub struct GetAuthRequest<'a, T: AsRef<str>> {
    pub access_token: T,
    pub options: Option<GetAuthRequestOptions<'a, T>>,
}

#[derive(Debug, Serialize)]
pub struct GetAuthRequestOptions<'a, T: AsRef<str>> {
    pub account_ids: &'a [T],
}

impl<T: AsRef<str> + serde::Serialize> Endpoint for GetAuthRequest<'_, T> {
    type Response = GetAuthResponse;
    fn path(&self) -> String {
        "/auth/get".into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAuthResponse {
    pub accounts: Vec<Account>,
    pub numbers: AccountNumbers,
    pub item: Item,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountNumbers {
    pub ach: Vec<ACHAccountNumber>,
    pub eft: Vec<EFTAccountNumber>,
    pub international: Vec<InternationalAccountNumber>,
    pub bacs: Vec<BACSAccountNumber>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ACHAccountNumber {
    pub account_id: String,
    pub account: String,
    pub routing: String,
    pub wire_routing: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EFTAccountNumber {
    pub account_id: String,
    pub account: String,
    pub institution: String,
    pub branch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternationalAccountNumber {
    pub account_id: String,
    /// The International Bank Account Number (IBAN) for the account.
    pub iban: String,
    /// The Bank Identifier Code (BIC) for the account
    pub bic: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BACSAccountNumber {
    pub account_id: String,
    pub account: String,
    pub sort_code: String,
}
