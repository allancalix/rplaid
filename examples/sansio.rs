use anyhow::Result;
use bytes::Bytes;
use http::{Request, Response};
use rplaid::{
    model::{CreateLinkTokenRequest, LinkUser},
    client::{Environment, Credentials},
    api::{Conf, create_link_token, create_link_token_response},
};

async fn http_send(client: &reqwest::Client, req: Request<Vec<u8>>) -> Result<Response<Bytes>> {
    let mut response = client
        .execute(req.try_into()?)
        .await?;

    let mut builder = http::Response::builder()
        .status(response.status())
        .version(response.version());

    std::mem::swap(builder.headers_mut().unwrap(), response.headers_mut());
    Ok(builder.body(response.bytes().await?)?)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cfg = Conf {
        credentials: Credentials {
            client_id: std::env::var("PLAID_CLIENT_ID").unwrap(),
            secret: std::env::var("PLAID_SECRET").unwrap(),
        },
        environment: Environment::Sandbox,
    };

    let req = CreateLinkTokenRequest {
        client_name: "test-client",
        country_codes: &["US"],
        language: "en",
        products: &["transactions"],
        user: LinkUser{
            client_user_id: "test-user",
            ..Default::default()
        },
        ..Default::default()
    };

    let http_req = create_link_token(&cfg, req).unwrap();

    let client = reqwest::Client::new();
    let response = http_send(&client, http_req).await.unwrap();
    let response = create_link_token_response(response).unwrap();

    println!("{:?}", response);
}
