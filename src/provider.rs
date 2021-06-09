use algonaut::{client::algod::v2::Client, core::Address};
use anyhow::Result;

pub struct Provider {
    client: Client,
}

pub struct AccountViewData {
    pub address: String,
    pub status: String,
    pub holdings: String,
    pub rewards: String,
    pub pending_rewards: String,
}

impl Provider {
    pub fn new(client: Client) -> Provider {
        Provider { client }
    }

    pub async fn get_infos(&self, address: &Address) -> Result<AccountViewData> {
        let account = self
            .client
            .account_information(&address.to_string())
            .await?;

        Ok(AccountViewData {
            address: account.address.to_string(),
            status: account.status,
            holdings: format!("{} microAlgos", account.amount.0),
            rewards: format!("{} microAlgos", account.rewards.0),
            pending_rewards: format!("{} microAlgos", account.pending_rewards.0),
        })
    }
}
