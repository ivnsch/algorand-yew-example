use algonaut::{algod::v2::Algod, core::Address};
use anyhow::Result;

pub struct Provider {
    algod: Algod,
}

pub struct AccountViewData {
    pub address: String,
    pub status: String,
    pub holdings: String,
    pub rewards: String,
    pub pending_rewards: String,
}

impl Provider {
    pub fn new(algod: Algod) -> Provider {
        Provider { algod }
    }

    pub async fn get_infos(&self, address: &Address) -> Result<AccountViewData> {
        let account = self.algod.account_information(address).await?;

        Ok(AccountViewData {
            address: account.address.to_string(),
            status: account.status,
            holdings: format!("{} microAlgos", account.amount.0),
            rewards: format!("{} microAlgos", account.rewards.0),
            pending_rewards: format!("{} microAlgos", account.pending_rewards.0),
        })
    }
}
