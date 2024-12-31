use payjp_client_core::{PayjpClient, BlockingClient, PayjpRequest, RequestBuilder, PayjpMethod};

    /// あなたのアカウント情報を取得します。
#[derive(Clone,Debug,)]#[derive(serde::Serialize)]
pub struct RetrieveForMyAccountAccount {

}
impl RetrieveForMyAccountAccount {
    /// Construct a new `RetrieveForMyAccountAccount`.
pub fn new() -> Self {
    Self {
        
    }
}

}
    impl Default for RetrieveForMyAccountAccount {
    fn default() -> Self {
        Self::new()
    }
}impl RetrieveForMyAccountAccount {
    /// Send the request and return the deserialized response.
    pub async fn send<C: PayjpClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: BlockingClient>(&self, client: &C) -> Result<<Self as PayjpRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    
}

impl PayjpRequest for RetrieveForMyAccountAccount {
    type Output = payjp_core::Account;

    fn build(&self) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, "/accounts")
}

}

