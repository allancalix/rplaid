use super::*;

#[derive(Debug, Serialize)]
pub struct SearchEmployerRequest<'a, T: AsRef<str>> {
    pub query: T,
    /// This field must be set to deposit_switch.
    pub products: &'a [T],
}

impl<T: AsRef<str> + HttpSerialize> Endpoint for SearchEmployerRequest<'_, T> {
    type Response = SearchEmployerResponse;

    fn path(&self) -> String {
        "/employers/search".into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchEmployerResponse {
    pub employers: Vec<Employer>,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employer {
    pub employer_id: String,
    pub name: String,
    pub address: Option<Address>,
    pub confidence_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub city: String,
    pub region: Option<String>,
    pub street: String,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}
