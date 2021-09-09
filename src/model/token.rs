use super::*;

#[derive(Debug, Serialize)]
pub struct ExchangePublicTokenRequest<T: AsRef<str>> {
    pub public_token: T,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for ExchangePublicTokenRequest<T> {
    type Response = ExchangePublicTokenResponse;

    fn path(&self) -> String {
        "/item/public_token/exchange".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangePublicTokenResponse {
    pub access_token: String,
    pub item_id: String,
    pub request_id: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateLinkTokenRequest<'a, T: AsRef<str>> {
    pub client_name: T,
    pub language: T,
    pub country_codes: &'a [T],
    pub user: LinkUser<T>,
    pub products: &'a [T],
    pub webhook: Option<T>,
    pub access_token: Option<T>,
    pub link_customization_name: Option<T>,
    pub redirect_uri: Option<T>,
    pub android_package_name: Option<T>,
    pub account_filters: Option<AccountFilters<'a, T>>,
    pub eu_config: Option<EUConfig>,
    pub payment_initiation: Option<PaymentInitiation<T>>,
    pub deposit_switch: Option<DepositSwitchOptions<T>>,
    pub income_verification: Option<IncomeVerification<T>>,
    pub auth: Option<LinkAuth<T>>,
    pub institution_id: Option<T>,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for CreateLinkTokenRequest<'_, T> {
    type Response = CreateLinkTokenResponse;

    fn path(&self) -> String {
        "/link/token/create".into()
    }
}

#[derive(Debug, Serialize, Default)]
pub struct LinkAuth<T: AsRef<str>> {
    flow_type: T,
}

#[derive(Debug, Serialize, Default)]
pub struct IncomeVerification<T: AsRef<str>> {
    income_verification_id: T,
    asset_report_id: Option<T>,
}

#[derive(Debug, Serialize, Default)]
pub struct DepositSwitchOptions<T: AsRef<str>> {
    deposit_switch_id: T,
}

#[derive(Debug, Serialize, Default)]
pub struct PaymentInitiation<T: AsRef<str>> {
    payment_id: T,
}

#[derive(Debug, Serialize, Default)]
pub struct LinkUser<T: AsRef<str>> {
    pub client_user_id: T,
    pub legal_name: Option<T>,
    pub phone_number: Option<T>,
    pub phone_number_verified_time: Option<T>,
    pub email_address: Option<T>,
    pub email_address_verified_time: Option<T>,
    pub ssn: Option<T>,
    pub date_of_birth: Option<T>,
}

#[derive(Debug, Serialize, Default)]
pub struct AccountFilters<'a, T: AsRef<str>> {
    depository: Option<AccountFilter<'a, T>>,
    credit: Option<AccountFilter<'a, T>>,
    loan: Option<AccountFilter<'a, T>>,
    investment: Option<AccountFilter<'a, T>>,
}

#[derive(Debug, Serialize, Default)]
pub struct EUConfig {
    headless: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct AccountFilter<'a, T: AsRef<str>> {
    account_subtypes: &'a [T],
}

impl<T: AsRef<str> + Default> LinkUser<T> {
    pub fn new(user_id: T) -> Self {
        Self {
            client_user_id: user_id,
            ..Self::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLinkTokenResponse {
    pub link_token: String,
    pub expiration: String,
    pub request_id: String,
}

#[derive(Debug, Serialize)]
pub struct GetLinkTokenRequest<T: AsRef<str>> {
    pub link_token: T,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for GetLinkTokenRequest<T> {
    type Response = GetLinkTokenResponse;

    fn path(&self) -> String {
        "/link/token/get".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLinkTokenResponse {
    pub link_token: String,
    pub expiration: Option<String>,
    pub created_at: Option<String>,
    pub request_id: String,
}

#[derive(Debug, Serialize)]
pub struct InvalidateAccessTokenRequest<T: AsRef<str>> {
    pub access_token: T,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for InvalidateAccessTokenRequest<T> {
    type Response = InvalidateAccessTokenResponse;

    fn path(&self) -> String {
        "/item/access_token/invalidate".into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InvalidateAccessTokenResponse {
    pub new_access_token: String,
    pub request_id: String,
}
