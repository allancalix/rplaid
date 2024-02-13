use http::{uri::Uri, header::CONTENT_TYPE, method::Method, Request, Response};

use crate::model::{
    CreateLinkTokenRequest, CreateLinkTokenResponse, ErrorResponse, Endpoint};
use crate::client::{Environment, Credentials, ClientError};

const HEADER_CLIENT_ID: &str = "PLAID-CLIENT-ID";
const HEADER_CLIENT_SECRET: &str = "PLAID-SECRET";

pub struct Conf {
    pub credentials: Credentials,
    pub environment: Environment,
}

type ClientRequestResult = Result<Request<Vec<u8>>, ClientError>;

pub fn create_link_token<'a>(cfg: &Conf, req: CreateLinkTokenRequest<&'a str>) -> ClientRequestResult {
    request(cfg, req)
}

pub fn create_link_token_response<T: AsRef<[u8]>>(res: Response<T>) -> Result<CreateLinkTokenResponse, ErrorResponse> {
    if res.status().is_success() {
        let body = res.body();
        return Ok(serde_json::from_slice(body.as_ref()).unwrap());
    }

    let body = res.body();
    Err(serde_json::from_slice(body.as_ref()).unwrap())
}

fn request(cfg: &Conf, endpoint: impl Endpoint) -> ClientRequestResult {
    let uri = Uri::builder()
        .scheme("https")
        .authority(cfg.environment.to_string())
        .path_and_query(endpoint.path())
        .build()?;

    let request_builder = Request::builder()
        .method(Method::POST)
        .header(HEADER_CLIENT_ID, &cfg.credentials.client_id)
        .header(HEADER_CLIENT_SECRET, &cfg.credentials.secret)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json");

    Ok(request_builder.body(serde_json::to_vec(&endpoint)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_link_token_req() {
        let client_cfg = Conf {
            credentials: Credentials {
                client_id: "fake-client-id".to_string(),
                secret: "client-secret".to_string(),
            },
            environment: Environment::Sandbox,
        };

        let req = CreateLinkTokenRequest {
            client_name: "test-client",
            country_codes: &["US"],
            products: &["transactions"],
            webhook: Some("http://localhost:4000/webhook/callback"),
            ..Default::default()
        };

        let http_req = create_link_token(&client_cfg, req).unwrap();

        assert_eq!(http_req.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap(), "application/json");
        assert_eq!(http_req.headers().get(HEADER_CLIENT_ID).unwrap().to_str().unwrap(), "fake-client-id");
        assert_eq!(http_req.headers().get(HEADER_CLIENT_SECRET).unwrap().to_str().unwrap(), "client-secret");
        assert_eq!(http_req.uri().authority().unwrap(), "sandbox.plaid.com");
        assert_eq!(http_req.uri().scheme().unwrap(), "https");
        assert_eq!(http_req.uri().path(), "/link/token/create");
    }
}
