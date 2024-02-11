use rplaid::client::{Builder, Credentials};
use rplaid::model::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = Builder::new()
        .with_credentials(Credentials {
            client_id: std::env::var("PLAID_CLIENT_ID").unwrap(),
            secret: std::env::var("PLAID_SECRET").unwrap(),
        })
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
