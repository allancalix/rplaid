# rplaid

[![crates.io](https://img.shields.io/crates/v/rplaid.svg)](https://crates.io/crates/rplaid)
[![Released API docs](https://docs.rs/rplaid/badge.svg)](https://docs.rs/rplaid)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![GHA Build Status](https://github.com/allancalix/rplaid/actions/workflows/test.yaml/badge.svg?branch=main)](https://github.com/allancalix/rplaid/actions?query=workflow%3ATEST)

**rplaid** is an async client for the [Plaid API](https://plaid.com/docs/api/).
With minimal features, the client is meant to be extensible and lightweight.
Additional features can be enabled to improve ergonomics of the API at the
cost of additional dependencies.

The goal is to provide expressive bindings that provide sensible defaults where
possible for ease of use.

See official [API docs](https://plaid.com/docs/) for more information about
endpoints or specific products.

__These are not official Plaid bindings.__

```rust
use rplaid::client::{Builder, Credentials};
use rplaid::model::*;

#[tokio::main]
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
```

## Glossary
* Item - A Item represents a connection to a single financial instution.
         Typically links are associated with a pair of credentials and an
         `access_token`. Items are associated to one or more accounts.

* Link - Link is a client-side component that Plaid provides to link to accounts.
         See https://plaid.com/docs/link/#introduction-to-link for more
         information.

* Account - An account is a financial account that is linked to an Item. An item,
            or financial institution, may have multiple accounts for a single
            user (e.g. a checking account and a credit account).

* Product - Entities with services offered by Plaid, see
            https://plaid.com/docs/api/products/ for more information.

## Features
* Idiomatic futures generator for easily reading multiple pages of transactions.
* Extensible HttpClient interfaces supports multiple HTTP clients with minimal
  effort (surf, H1, and reqwest). The trait can also be implemented to have full
  control over the HTTP client used.
* Rust types, including variant types, for most API return types.

## Limitations
Some endpoints are production specific or beta products and are not yet
supported by the client.

For a breakdown of endpoint support visit:
https://docs.google.com/spreadsheets/d/1xqUXdfllo37Rx5MVrQODbVqNQvuktiCVL5Uh8y9mYYw

## License
[MIT](LICENSE)
