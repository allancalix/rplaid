use super::*;

#[derive(Debug, Serialize)]
pub struct AccountBalancesGetRequest<'a, T: AsRef<str>> {
    pub access_token: T,
    pub options: Option<AccountBalanceFilter<'a, T>>,
}

#[derive(Debug, Serialize)]
pub struct AccountBalanceFilter<'a, T: AsRef<str>> {
    pub account_ids: &'a [T],
    pub min_last_updated_datetime: Option<T>,
}

impl<T: AsRef<str> + serde::Serialize> Endpoint for AccountBalancesGetRequest<'_, T> {
    type Response = AccountBalancesGetResponse;

    fn path(&self) -> String {
        "/accounts/balance/get".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountBalancesGetResponse {
    pub accounts: Vec<Account>,
    pub item: Item,
    pub request_id: String,
}
