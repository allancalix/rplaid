use http_client::h1::H1Client;
use http_client::Error as HttpError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::*;
use crate::HttpClient;

const SANDBOX_DOMAIN: &str = "https://sandbox.plaid.com";
const DEVELOPMENT_DOMAIN: &str = "https://development.plaid.com";
const PRODUCTION_DOMAIN: &str = "https://production.plaid.com";

/// Error returned by client requests.
#[derive(Error, Debug)]
pub enum ClientError {
    /// Wraps errors from the underlying HTTP client.
    #[error("http request failed: {0}")]
    Http(HttpError),
    /// Error either serializing request types or deserializing response types
    /// from requests.
    #[error(transparent)]
    Parse(#[from] serde_json::Error),
    /// Wraps errors from Plaid's API responses. If an error is parsed then
    /// Plaid successfully returned a response but returned with errors.
    #[error(transparent)]
    App(#[from] ErrorResponse),
}

/// Credentials required to make authenticated calls to the Plaid API.
#[derive(Debug, Default)]
pub struct Credentials {
    /// Plaid API client id token.
    pub client_id: String,
    /// Plaid API secret for the configured environment (e.g. sandbox, dev, prod).
    pub secret: String,
}

impl From<HttpError> for ClientError {
    fn from(error: HttpError) -> Self {
        Self::Http(error)
    }
}

/// Environment controls the domain for the client, matches Plaid's sandbox,
/// development, and production environments.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Environment {
    /// Used to configure the client to request against a the domain in the string.
    /// Should be a fully qualified domain with protocol and scheme, for example
    /// http://localhost:3000.
    Custom(String),
    /// Plaid sandbox environment.
    Sandbox,
    /// Plaid development environment.
    Development,
    /// Plaid production environment.
    Production,
}

impl std::default::Default for Environment {
    fn default() -> Self {
        Environment::Sandbox
    }
}

impl std::string::ToString for Environment {
    fn to_string(&self) -> String {
        match self {
            Environment::Sandbox => SANDBOX_DOMAIN.into(),
            Environment::Development => DEVELOPMENT_DOMAIN.into(),
            Environment::Production => PRODUCTION_DOMAIN.into(),
            Environment::Custom(s) => s.into(),
        }
    }
}

/// Plaid API client type.
pub struct Plaid<T: HttpClient> {
    http: T,
    credentials: Credentials,
    env: Environment,
}

/// Builder helps construct Plaid client types with sensible defaults.
pub struct Builder {
    http: Option<Box<dyn HttpClient>>,
    credentials: Option<Credentials>,
    env: Option<Environment>,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Constructs a new Plaid client builder.
    ///
    /// ```
    /// use rplaid::client::Builder;
    ///
    /// let client = Builder::new().build();
    /// ```
    pub fn new() -> Self {
        Self {
            http: None,
            credentials: None,
            env: None,
        }
    }

    /// Override the default HTTP client.
    pub fn with_http_client(mut self, client: impl HttpClient) -> Self {
        self.http = Some(Box::new(client));
        self
    }

    /// Set Plaid API credentials for authenticating Plaid API calls.
    pub fn with_credentials(mut self, creds: Credentials) -> Self {
        self.credentials = Some(creds);
        self
    }

    /// Set API request environment.
    pub fn with_env(mut self, env: Environment) -> Self {
        self.env = Some(env);
        self
    }

    /// Consume a builder returning a Plaid client instance.
    pub fn build(self) -> Plaid<Box<dyn HttpClient>> {
        let http = self.http.unwrap_or_else(|| Box::new(H1Client::new()));
        Plaid {
            http,
            credentials: self.credentials.unwrap_or_default(),
            env: self.env.unwrap_or_default(),
        }
    }
}

impl<T: HttpClient> Plaid<T> {
    /// Creates a new Plaid instance from a set of credentials and an HttpClient.
    ///
    /// ```
    /// use rplaid::client::{Plaid, Credentials, Environment};
    /// use http_client::h1::H1Client;
    ///
    /// let client = Plaid::new(
    ///     H1Client::new(),
    ///     Credentials{client_id: "".into(), secret: "".into()},
    ///     Environment::Sandbox);
    /// ```
    pub fn new(http: T, credentials: Credentials, env: Environment) -> Self {
        Self {
            http,
            credentials,
            env,
        }
    }

    async fn request<E: crate::model::Endpoint>(
        &self,
        endpoint: &E,
    ) -> Result<E::Response, ClientError>
    where
        for<'de> <E as crate::model::Endpoint>::Response: serde::Deserialize<'de>,
    {
        let mut post = endpoint.request(&self.env.to_string());
        post.insert_header("Content-Type", "application/json");
        post.insert_header("PLAID-CLIENT-ID", &self.credentials.client_id);
        post.insert_header("PLAID-SECRET", &self.credentials.secret);
        let mut res = self.http.send(post).await?;

        match res.status() {
            http_client::http_types::StatusCode::Ok => Ok(res.body_json::<E::Response>().await?),
            _ => Err(ClientError::from(res.body_json::<ErrorResponse>().await?)),
        }
    }

    /// Returns details for institutions that match the query parameters up to a
    /// maximum of ten institutions per query.
    ///
    /// https://plaid.com/docs/api/institutions/#institutionssearch
    pub async fn search_institutions<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &InstitutionsSearchRequest<'_, P>,
    ) -> Result<Vec<Institution>, ClientError> {
        Ok(self.request(req).await?.institutions)
    }

    /// Returns details on an institution currently supported by plaid.
    ///
    /// https://plaid.com/docs/api/institutions/#institutionsget_by_id
    pub async fn get_institution_by_id<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &InstitutionGetRequest<'_, P>,
    ) -> Result<Institution, ClientError> {
        Ok(self.request(req).await?.institution)
    }

    /// Returns details on all financial institutions currently supported by
    /// Plaid. Plaid supports thousands of institutions and results are
    /// paginated. Institutions with no overlap to the client's enabled products
    /// are filtered from results.
    ///
    /// https://plaid.com/docs/api/institutions/#institutionsget
    pub async fn get_institutions<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &InstitutionsGetRequest<'_, P>,
    ) -> Result<Vec<Institution>, ClientError> {
        Ok(self.request(req).await?.institutions)
    }

    /// Creates a valid `public_token` for an institution ID, initial products,
    /// and test credentials. The created public token maps to a new Sandbox
    /// item.
    ///
    /// https://plaid.com/docs/api/sandbox/#sandboxpublic_tokencreate
    pub async fn create_public_token<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: CreatePublicTokenRequest<'_, P>,
    ) -> Result<String, ClientError> {
        Ok(self.request(&req).await?.public_token)
    }

    /// Forces an item into an `ITEM_LOGIN_REQUIRED` state in order to simulate
    /// an Item whose login is no longer valid.
    ///
    /// https://plaid.com/docs/api/sandbox/#sandboxitemreset_login
    pub async fn reset_login<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        access_token: P,
    ) -> Result<(), ClientError> {
        let res = self.request(&ResetLoginRequest { access_token }).await?;
        match res.reset_login {
            true => Ok(()),
            false => Err(ClientError::App(ErrorResponse {
                error_message: Some("failed to reset login".into()),
                ..ErrorResponse::default()
            })),
        }
    }

    /// Exchange a Link `public_token` for an API `access_token`. Public tokens
    /// are ephemeral and expires after 30 minutes.
    ///
    /// https://plaid.com/docs/api/tokens/#itempublic_tokenexchange
    pub async fn exchange_public_token<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        public_token: P,
    ) -> Result<ExchangePublicTokenResponse, ClientError> {
        Ok(self
            .request(&ExchangePublicTokenRequest { public_token })
            .await?)
    }

    /// Creates a `link_token` that is required as a parameter when initializing
    /// a Link.
    ///
    /// https://plaid.com/docs/api/tokens/#linktokencreate
    pub async fn create_link_token<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &CreateLinkTokenRequest<'_, P>,
    ) -> Result<CreateLinkTokenResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Retrieves information for any linked item, only active accounts are
    /// returned. Responses may be cached, if up-to-date information is required
    /// use `balances` instead.
    ///
    /// https://plaid.com/docs/api/accounts/#accountsget
    pub async fn accounts<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        access_token: P,
    ) -> Result<Vec<Account>, ClientError> {
        Ok(self
            .request(&GetAccountsRequest {
                access_token,
                options: None,
            })
            .await?
            .accounts)
    }

    /// Returns information about the status of an Item.
    ///
    /// https://plaid.com/docs/api/items/#itemget
    pub async fn item<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        access_token: P,
    ) -> Result<Item, ClientError> {
        Ok(self.request(&GetItemRequest { access_token }).await?.item)
    }

    /// Removes an Item. Once removed, the `access_token` associated with the
    /// Item is no longer valid and cannot be used to access any data that was
    /// associated with the Item.
    ///
    /// https://plaid.com/docs/api/items/#itemremove
    pub async fn item_del<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        access_token: P,
    ) -> Result<(), ClientError> {
        self.request(&RemoveItemRequest { access_token }).await?;

        Ok(())
    }

    /// Updates the webhook URL associated with an Item. Updates trigger a
    /// `WEBHOOK_UPDATE_ACKNOWLEDGED` event to the new webhook URL.
    ///
    /// https://plaid.com/docs/api/items/#itemwebhookupdate
    pub async fn item_webhook_update<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        access_token: P,
        webhook: P,
    ) -> Result<Item, ClientError> {
        Ok(self
            .request(&UpdateItemWebhookRequest {
                access_token,
                webhook,
            })
            .await?
            .item)
    }

    /// Verify real-time account balances. This endpoint can be used as long as
    /// Link has been initialized with any other product.
    ///
    /// https://plaid.com/docs/api/products/#balance
    pub async fn balances<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        access_token: P,
    ) -> Result<Vec<Account>, ClientError> {
        Ok(self
            .request(&AccountBalancesGetRequest {
                access_token,
                options: None,
            })
            .await?
            .accounts)
    }

    /// Returns the bank account and bank identification numbers associated with
    /// an Item's checking and savings accounts, along with high-level account
    /// data and balances when available.
    ///
    /// https://plaid.com/docs/api/products/#auth
    pub async fn auth<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &GetAuthRequest<'_, P>,
    ) -> Result<GetAuthResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Verify the name, address, phone number, and email address of a user
    /// against bank account information on file.
    ///
    /// https://plaid.com/docs/api/products/#identity
    pub async fn identity<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &GetIdentityRequest<'_, P>,
    ) -> Result<GetIdentityResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Triggers a Transactions `DEFAULT_UPDATE` webhook for a given Sandbox
    /// Item. If the Item does not support Transactions, a
    /// `SANDBOX_PRODUCT_NOT_ENABLED` error will result.
    ///
    /// https://plaid.com/docs/api/sandbox/#sandboxitemfire_webhook
    pub async fn fire_webhook<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &FireWebhookRequest<P>,
    ) -> Result<FireWebhookResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Changes the verification status of an Item in the sandbox in order to
    /// simulate the Automated Micro-deposit flow.
    ///
    /// https://plaid.com/docs/api/sandbox/#sandboxitemset_verification_status
    pub async fn set_verification_status<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &SetVerificationStatusRequest<P>,
    ) -> Result<SetVerificationStatusResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Searches Plaid's database for known employers to use with Deposit
    /// Switch.
    ///
    /// https://plaid.com/docs/api/employers/
    pub async fn search_employers<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &SearchEmployerRequest<'_, P>,
    ) -> Result<SearchEmployerResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Provides a JSON Web Key (JWK) that can be used to verify a JWT.
    ///
    /// https://plaid.com/docs/api/webhooks/webhook-verification/#webhook_verification_keyget
    pub async fn create_webhook_verification_key<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &GetWebhookVerificationKeyRequest<P>,
    ) -> Result<GetWebhookVerificationKeyResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Gets information about a `link_token`, can be useful for debugging.
    ///
    /// https://plaid.com/docs/api/tokens/#linktokenget
    pub async fn link_token<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &GetLinkTokenRequest<P>,
    ) -> Result<GetLinkTokenResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Rotate the `access_token` associated with an Item. Call returns a new
    /// `access_token` and immediately invalidates the previous token.
    ///
    /// https://plaid.com/docs/api/tokens/#itemaccess_tokeninvalidate
    pub async fn invalidate_access_token<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &InvalidateAccessTokenRequest<P>,
    ) -> Result<InvalidateAccessTokenResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Get detailed information on categories returned by Plaid. This endpoint
    /// does not require authentication.
    ///
    /// https://plaid.com/docs/api/products/#categoriesget
    pub async fn categories(
        &self,
        req: &GetCategoriesRequest,
    ) -> Result<GetCategoriesResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Initiates on-demand extraction to fetch the newest transactions for an
    /// Item.
    ///
    /// https://plaid.com/docs/api/products/#transactionsrefresh
    pub async fn refresh_transactions<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &RefreshTransactionsRequest<P>,
    ) -> Result<(), ClientError> {
        self.request(req).await?;
        Ok(())
    }

    /// Returns user-authorized transaction data for credit, depository, and
    /// some loan-type accounts. For transaction history from investment
    /// accounts, use investment endpoints instead (production only). Results
    /// are paginated based on request options and defaults to 100 entities per
    /// page.
    ///
    /// https://plaid.com/docs/api/products/#transactionsget
    pub async fn transactions<P: AsRef<str> + http_types::convert::Serialize>(
        &self,
        req: &GetTransactionsRequest<P>,
    ) -> Result<GetTransactionsResponse, ClientError> {
        Ok(self.request(req).await?)
    }

    /// Returns a Stream of transactions that can be used to iterative fetch
    /// pages from the transaction endpoint. Each call will return the number of
    /// items configured in the original request.
    ///
    /// ```ignore
    /// use futures_util::pin_mut;
    /// use futures_util::StreamExt;
    ///
    /// ...
    ///
    ///   let req = GetTransactionsRequest {
    ///       access_token: res.access_token.as_str(),
    ///       start_date: "2019-09-01",
    ///       end_date: "2021-09-05",
    ///       options: Some(GetTransactionsOptions {
    ///           // Number of items to return per page.
    ///           count: Some(10),
    ///           // Number of items from the start_date to offset results by.
    ///           offset: Some(5),
    ///           account_ids: None,
    ///           include_original_description: None,
    ///       }),
    ///   };
    ///   let iter = client.transactions_iter(req);
    ///   pin_mut!(iter);
    ///
    ///   while let Some(txn) = iter.next().await {
    ///     println!("{:?}", txn);
    ///   }
    /// ```
    #[cfg(feature = "streams")]
    pub fn transactions_iter<'a, P: AsRef<str> + http_types::convert::Serialize + Clone + 'a>(
        &'a self,
        req: GetTransactionsRequest<P>,
    ) -> impl futures_core::stream::Stream<Item = Result<Vec<Transaction>, ClientError>> + 'a {
        async_stream::try_stream! {
            let mut yielded = 0;
            let mut total_xacts = None;
            let mut request = req.clone();
            let count = req.options.as_ref().unwrap().count.unwrap_or(100);
            let mut offset = req.options.as_ref().unwrap().offset.unwrap_or(0);

            while total_xacts.is_none() || total_xacts.unwrap() > yielded {
                if let Some(ref mut opts) = &mut request.options {
                    opts.count = Some(count);
                    opts.offset = Some(offset);
                } else {
                    request.options = Some(GetTransactionsOptions{
                        count: Some(count),
                        offset: Some(offset),
                        account_ids: None,
                        include_original_description: None,
                    });
                }

                let res = self.transactions(&request).await?;
                if total_xacts.is_none() {
                    total_xacts = Some(res.total_transactions - offset);
                }
                yielded += res.transactions.len();
                offset += yielded;

                yield res.transactions;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::pin_mut;
    use futures_util::StreamExt;

    const INSTITUTION_ID: &str = "ins_129571";

    fn credentials() -> Credentials {
        Credentials {
            client_id: std::env::var("PLAID_CLIENT_ID")
                .expect("Variable PLAID_CLIENT_ID must be defined."),
            secret: std::env::var("PLAID_SECRET").expect("Variable PLAID_SECRET must be defined."),
        }
    }

    #[tokio::test]
    async fn unauthorized_calls_return_parsable_error() {
        let client = Builder::new().with_credentials(credentials()).build();
        let res = client
            // Accounts is an authenticated call and requires a valid access token.
            .accounts("")
            .await;

        match res.unwrap_err() {
            ClientError::App(e) => {
                assert_eq!(e.error_type.unwrap(), ErrorType::InvalidRequest);
            }
            _ => panic!("unexpected error"),
        }
    }

    #[tokio::test]
    async fn can_get_multiple_institutions() {
        let client = Builder::new().with_credentials(credentials()).build();
        let res = client
            .get_institutions(&InstitutionsGetRequest {
                count: 10,
                offset: 0,
                country_codes: &["US"],
                options: None,
            })
            .await
            .unwrap();

        insta::assert_json_snapshot!(res);
    }

    #[tokio::test]
    async fn can_fetch_single_institution() {
        let client = Builder::new().with_credentials(credentials()).build();
        let res = client
            .get_institution_by_id(&InstitutionGetRequest {
                institution_id: INSTITUTION_ID,
                country_codes: &[],
                options: None,
            })
            .await
            .unwrap();

        insta::assert_json_snapshot!(res);
    }

    #[tokio::test]
    async fn can_search_institutions() {
        let client = Builder::new().with_credentials(credentials()).build();
        let res = client
            .search_institutions(&InstitutionsSearchRequest {
                query: "Banque Populaire",
                country_codes: &[],
                products: None,
                options: None,
            })
            .await
            .unwrap();

        insta::assert_json_snapshot!(res);
    }

    #[tokio::test]
    async fn can_create_sandbox_pub_token() {
        let client = Builder::new().with_credentials(credentials()).build();
        let public_token = client
            .create_public_token(CreatePublicTokenRequest {
                institution_id: INSTITUTION_ID,
                initial_products: &["assets", "auth", "balance"],
                options: None,
            })
            .await
            .unwrap();

        let res = client.exchange_public_token(public_token).await.unwrap();
        assert!(!res.access_token.is_empty());
        // Should succeed.
        client.reset_login(res.access_token).await.unwrap();
    }

    #[tokio::test]
    async fn can_fetch_accounts_with_token() {
        let client = Builder::new().with_credentials(credentials()).build();
        let public_token = client
            .create_public_token(CreatePublicTokenRequest {
                institution_id: INSTITUTION_ID,
                initial_products: &["assets", "auth", "balance"],
                options: None,
            })
            .await
            .unwrap();

        let res = client.exchange_public_token(public_token).await.unwrap();
        assert!(!res.access_token.is_empty());
        let accounts = client.accounts(res.access_token).await.unwrap();

        insta::assert_json_snapshot!(accounts, {
            "[].account_id" => "[account_id]"
        });
    }

    #[tokio::test]
    async fn can_modify_items() {
        let client = Builder::new().with_credentials(credentials()).build();
        let public_token = client
            .create_public_token(CreatePublicTokenRequest {
                institution_id: INSTITUTION_ID,
                initial_products: &["assets", "auth", "balance"],
                options: None,
            })
            .await
            .unwrap();

        let res = client.exchange_public_token(public_token).await.unwrap();
        assert!(!res.access_token.is_empty());
        let item = client.item(&res.access_token).await.unwrap();

        insta::assert_json_snapshot!(item, {
            ".item_id" => "[item_id]"
        });

        // Should succeed.
        client.item_del(res.access_token).await.unwrap();
    }

    #[tokio::test]
    async fn can_create_link_token() {
        let client = Builder::new().with_credentials(credentials()).build();
        let create_res = client
            .create_link_token(&CreateLinkTokenRequest {
                client_name: "test_client",
                user: LinkUser::new("test-user"),
                language: "en",
                country_codes: &["US"],
                products: &["transactions"],
                ..CreateLinkTokenRequest::default()
            })
            .await
            .unwrap();

        assert!(!create_res.link_token.is_empty());
        // Check that we can read back the token we created.
        let res = client
            .link_token(&GetLinkTokenRequest {
                link_token: &create_res.link_token,
            })
            .await
            .unwrap();
        assert_eq!(create_res.link_token, res.link_token);
    }

    #[tokio::test]
    async fn can_read_transactions() {
        let client = Builder::new().with_credentials(credentials()).build();
        let public_token = client
            .create_public_token(CreatePublicTokenRequest {
                institution_id: INSTITUTION_ID,
                initial_products: &["assets", "auth", "balance", "transactions"],
                options: None,
            })
            .await
            .unwrap();

        let res = client.exchange_public_token(public_token).await.unwrap();
        assert!(!res.access_token.is_empty());
        // TODO(allancalix): Transaction isn't available immediately after the
        // token is created, we probably want to find a better way to find out if
        // the product is ready.
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        let res = client
            .transactions(&GetTransactionsRequest {
                access_token: res.access_token.as_str(),
                start_date: "2021-09-01",
                end_date: "2021-09-05",
                options: None,
            })
            .await
            .unwrap();

        insta::assert_json_snapshot!(res.transactions, {
            "[].transaction_id" => "[transaction_id]",
            "[].account_id" => "[account_id]",
        });
    }

    #[tokio::test]
    async fn can_drain_transaction_stream() {
        let client = Builder::new().with_credentials(credentials()).build();
        let public_token = client
            .create_public_token(CreatePublicTokenRequest {
                institution_id: INSTITUTION_ID,
                initial_products: &["assets", "auth", "balance", "transactions"],
                options: None,
            })
            .await
            .unwrap();

        let res = client.exchange_public_token(public_token).await.unwrap();
        assert!(!res.access_token.is_empty());
        // TODO(allancalix): Transaction isn't available immediately after the
        // token is created, we probably want to find a better way to find out if
        // the product is ready.
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let req = GetTransactionsRequest {
            access_token: res.access_token.as_str(),
            start_date: "2019-09-01",
            end_date: "2021-09-05",
            options: Some(GetTransactionsOptions {
                count: Some(10),
                offset: Some(5),
                account_ids: None,
                include_original_description: None,
            }),
        };
        let iter = client.transactions_iter(req);
        pin_mut!(iter);

        let xacts = iter
            .fold(vec![], |mut acc, x| async move {
                acc.append(&mut x.unwrap());
                acc
            })
            .await;
        assert_eq!(xacts.len(), 7);
    }

    #[tokio::test]
    async fn can_read_categories() {
        let client = Builder::new().with_credentials(credentials()).build();
        let res = client.categories(&GetCategoriesRequest {}).await.unwrap();
        insta::assert_json_snapshot!(res.categories);
    }

    #[tokio::test]
    async fn can_refresh_transactions() {
        let client = Builder::new().with_credentials(credentials()).build();
        let public_token = client
            .create_public_token(CreatePublicTokenRequest {
                institution_id: INSTITUTION_ID,
                initial_products: &["assets", "auth", "balance", "transactions"],
                options: None,
            })
            .await
            .unwrap();
        let res = client.exchange_public_token(public_token).await.unwrap();
        assert!(!res.access_token.is_empty());

        client
            .refresh_transactions(&RefreshTransactionsRequest {
                access_token: res.access_token,
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn can_read_auth() {
        let client = Builder::new().with_credentials(credentials()).build();
        let public_token = client
            .create_public_token(CreatePublicTokenRequest {
                institution_id: INSTITUTION_ID,
                initial_products: &["assets", "auth", "balance", "transactions"],
                options: None,
            })
            .await
            .unwrap();
        let res = client.exchange_public_token(public_token).await.unwrap();
        assert!(!res.access_token.is_empty());

        let res = client
            .auth(&GetAuthRequest {
                access_token: res.access_token,
                options: None,
            })
            .await
            .unwrap();
        insta::assert_json_snapshot!(res, {
            ".accounts[].account_id" => "[account_id]",
            ".numbers.ach[].account_id" => "[ach_account_id]",
            ".request_id" => "[request_id]",
            ".item.item_id" => "[item_id]",
        });
    }

    #[tokio::test]
    async fn can_read_identity() {
        let client = Builder::new().with_credentials(credentials()).build();
        let public_token = client
            .create_public_token(CreatePublicTokenRequest {
                institution_id: INSTITUTION_ID,
                initial_products: &["assets", "auth", "balance", "transactions"],
                options: None,
            })
            .await
            .unwrap();
        let res = client.exchange_public_token(public_token).await.unwrap();
        assert!(!res.access_token.is_empty());

        let res = client
            .identity(&GetIdentityRequest {
                access_token: res.access_token,
                options: None,
            })
            .await
            .unwrap();
        insta::assert_json_snapshot!(res, {
            ".accounts[].account_id" => "[account_id]",
            ".item.item_id" => "[item_id]",
            ".request_id" => "[request_id]",
        });
    }

    #[tokio::test]
    async fn can_invalidate_access_token() {
        let client = Builder::new().with_credentials(credentials()).build();
        let public_token = client
            .create_public_token(CreatePublicTokenRequest {
                institution_id: INSTITUTION_ID,
                initial_products: &["assets", "auth", "balance", "transactions"],
                options: None,
            })
            .await
            .unwrap();
        let create_res = client.exchange_public_token(public_token).await.unwrap();
        assert!(!create_res.access_token.is_empty());

        let res = client
            .invalidate_access_token(&InvalidateAccessTokenRequest {
                access_token: &create_res.access_token,
            })
            .await
            .unwrap();
        // A new access token should be returned.
        assert_ne!(res.new_access_token, create_res.access_token);
    }

    #[tokio::test]
    async fn can_fire_webhook() {
        let client = Builder::new().with_credentials(credentials()).build();
        let public_token = client
            .create_public_token(CreatePublicTokenRequest {
                institution_id: INSTITUTION_ID,
                initial_products: &["assets", "auth", "balance", "transactions"],
                options: Some(CreatePublicTokenOptions {
                    webhook: Some("localhost:3000"),
                    override_username: None,
                    override_password: None,
                    transactions: None,
                }),
            })
            .await
            .unwrap();
        let res = client.exchange_public_token(public_token).await.unwrap();
        let res = client
            .fire_webhook(&FireWebhookRequest {
                access_token: res.access_token.as_str(),
                webhook_code: WebhookCode::DefaultUpdate,
            })
            .await
            .unwrap();

        assert!(res.webhook_fired);
    }
}
