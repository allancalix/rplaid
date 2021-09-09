use http_client::isahc::IsahcClient;
use rplaid::client::{Builder, Credentials};
use rplaid::model::*;

#[tokio::main]
async fn main() {
    let client = Builder::new()
        .with_credentials(Credentials {
            client_id: std::env::var("PLAID_CLIENT_ID").unwrap(),
            secret: std::env::var("PLAID_SECRET").unwrap(),
        })
        .with_http_client(IsahcClient::new())
        .build();
    let institutions = client
        .get_institutions(&InstitutionsGetRequest {
            count: 10,
            offset: 0,
            country_codes: &["US"],
            options: None,
        })
        .await
        .unwrap();

    println!("{:?}", institutions);
}
