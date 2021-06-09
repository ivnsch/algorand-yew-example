use algonaut::client::algod::v2::Client;
use algonaut::Algod;
use anyhow::Result;

use crate::provider::Provider;

pub fn client() -> Result<Client> {
    let client = Algod::new()
        .bind("http://127.0.0.1:53630")
        .auth("44d70009a00561fe340b2584a9f2adc6fec6a16322554d44f56bef9e682844b9")
        .client_v2()?;
    Ok(client)
}

pub fn provider(client: Client) -> Provider {
    Provider::new(client)
}
