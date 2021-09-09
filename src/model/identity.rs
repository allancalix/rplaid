use super::*;

#[derive(Debug, Serialize)]
pub struct GetIdentityRequest<'a, T: AsRef<str>> {
    pub access_token: T,
    pub options: Option<IdentityFilter<'a, T>>,
}

#[derive(Debug, Serialize)]
pub struct IdentityFilter<'a, T: AsRef<str>> {
    pub account_ids: &'a [T],
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for GetIdentityRequest<'_, T> {
    type Response = GetIdentityResponse;

    fn path(&self) -> String {
        "/identity/get".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetIdentityResponse {
    pub accounts: Vec<Account>,
    pub item: Item,
    pub request_id: String,
}
