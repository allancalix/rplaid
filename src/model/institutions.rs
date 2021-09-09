use super::*;

#[derive(Debug, Serialize)]
pub struct InstitutionsSearchRequest<'a, T: AsRef<str>> {
    pub query: T,
    pub products: Option<&'a [T]>,
    pub country_codes: &'a [T],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SearchInstitutionFilter<T>>,
}

#[derive(Debug, Serialize)]
pub struct SearchInstitutionFilter<T: AsRef<str>> {
    pub oauth: Option<bool>,
    pub include_optional_metadata: Option<bool>,
    pub include_auth_metadata: Option<bool>,
    pub include_payment_initiation_metadata: Option<bool>,
    pub payment_initiation: Option<PaymentInitiationFilter<T>>,
}

#[derive(Debug, Serialize)]
pub struct PaymentInitiationFilter<T: AsRef<str>> {
    pub payment_id: Option<T>,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for InstitutionsSearchRequest<'_, T> {
    type Response = InstitutionSearchResponse;
    fn path(&self) -> String {
        "/institutions/search".into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionSearchResponse {
    pub institutions: Vec<Institution>,
}

#[derive(Debug, Serialize)]
pub struct InstitutionGetRequest<'a, T: AsRef<str>> {
    pub institution_id: T,
    pub country_codes: &'a [T],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GetInstitutionFilter>,
}

#[derive(Debug, Serialize)]
pub struct GetInstitutionFilter {
    pub include_optional_metadata: Option<bool>,
    pub include_status: Option<bool>,
    pub include_auth_metadata: Option<bool>,
    pub include_payment_initiation_metadata: Option<bool>,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for InstitutionGetRequest<'_, T> {
    type Response = InstitutionGetResponse;
    fn path(&self) -> String {
        "/institutions/get_by_id".into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionGetResponse {
    pub institution: Institution,
}

#[derive(Debug, Serialize)]
pub struct InstitutionsGetRequest<'a, T: AsRef<str>> {
    pub count: usize,
    pub offset: usize,
    pub country_codes: &'a [T],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GetInstitutionsFilter<'a, T>>,
}

#[derive(Debug, Serialize)]
pub struct GetInstitutionsFilter<'a, T: AsRef<str>> {
    /// Filter the Institutions based on which products they support.
    pub products: &'a [T],
    /// Specify an array of routing numbers to filter institutions. The
    /// response will only return institutions that match all of the routing
    /// numbers in the array.
    pub routing_numbers: &'a [T],
    pub oauth: bool,
    pub include_optional_metadata: bool,
    /// When true, returns metadata related to the Auth product indicating
    /// which auth methods are supported. Defaults to false.
    pub include_auth_metadata: bool,
    /// When true, returns metadata related to the Payment Initiation product
    /// indicating which payment configurations are supported. Defaults to
    /// false.
    pub include_payment_initiation_metadata: bool,
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for InstitutionsGetRequest<'_, T> {
    type Response = InstitutionsGetResponse;
    fn path(&self) -> String {
        "/institutions/get".into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetResponse {
    pub institutions: Vec<Institution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Institution {
    pub institution_id: String,
    pub name: String,
    pub products: Vec<String>,
    pub country_codes: Vec<String>,
    pub url: Option<String>,
    pub primary_color: Option<String>,
    pub logo: Option<String>,
    pub routing_numbers: Option<Vec<String>>,
    pub oauth: bool,
}
