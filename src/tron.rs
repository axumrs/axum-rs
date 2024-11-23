use crate::{config, utils, Error, Result};

use rust_decimal::Decimal;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionKind {
    #[default]
    TRX,
    USDT,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBase {
    pub contract_ret: String,
    pub confirmed: bool,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionTRX {
    #[serde(flatten)]
    pub base: TransactionBase,
    pub contract_data: ContractData,
}

impl TransactionTRX {
    pub fn is_confirmed(&self) -> bool {
        self.base.contract_ret == "SUCCESS" && self.base.confirmed
    }

    pub fn amount(&self) -> Decimal {
        Decimal::from_i128_with_scale(self.contract_data.amount as i128, 0)
            / Decimal::from_i128_with_scale(1000000, 0)
    }

    pub fn is_to_my(&self, addr: &str) -> bool {
        self.contract_data.to_address == addr
    }

    pub fn is_valid(&self, cfg: &config::TronConfig, amount: &Decimal) -> Result<bool> {
        if !self.is_confirmed() {
            return Err(Error::new("交易未确认"));
        }

        if !self.is_to_my(&cfg.wallet) {
            return Err(Error::new("收款地址错误"));
        }

        if &self.amount() != amount {
            return Err(Error::new("金额错误"));
        }
        Ok(true)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionTRC20 {
    #[serde(flatten)]
    pub base: TransactionBase,
    pub contract_data: ContractDataTrc20,
    #[serde(rename = "trc20TransferInfo")]
    pub trc20transfer_info: Vec<Trc20TransferInfo>,
}

impl TransactionTRC20 {
    pub fn is_confirmed(&self, contract_addr: &str) -> bool {
        self.base.contract_ret == "SUCCESS"
            && self.base.confirmed
            && self.trc20transfer_info.len() > 0
            && self.contract_data.contract_address == contract_addr
    }

    pub fn amount(&self) -> Decimal {
        Decimal::from_str_exact(self.trc20transfer_info[0].amount_str.as_str()).unwrap_or_default()
            / Decimal::from_i128_with_scale(1000000, 0)
    }

    pub fn is_to_my(&self, addr: &str) -> bool {
        self.trc20transfer_info[0].to_address == addr
    }

    pub fn is_valid(&self, cfg: &config::TronConfig, amount: &Decimal) -> Result<bool> {
        if !self.is_confirmed(&cfg.usdt_contract_addr) {
            return Err(Error::new("交易未确认"));
        }

        if !self.is_to_my(&cfg.wallet) {
            return Err(Error::new("收款地址错误"));
        }

        if &self.amount() != amount {
            return Err(Error::new("金额错误"));
        }
        Ok(true)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractData {
    pub amount: i64,
    #[serde(rename = "owner_address")]
    pub owner_address: String,
    #[serde(rename = "to_address")]
    pub to_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDataTrc20 {
    pub data: String,
    #[serde(rename = "owner_address")]
    pub owner_address: String,
    #[serde(rename = "contract_address")]
    pub contract_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trc20TransferInfo {
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    pub symbol: String,
    pub level: String,
    #[serde(rename = "to_address")]
    pub to_address: String,
    #[serde(rename = "contract_address")]
    pub contract_address: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub decimals: i64,
    pub name: String,
    pub vip: bool,
    pub token_type: String,
    #[serde(rename = "from_address")]
    pub from_address: String,
    #[serde(rename = "amount_str")]
    pub amount_str: String,
    pub status: i64,
}

async fn _get_transaction_info(
    txid: &str,
    api_url: &str,
    timeout: u8,
) -> reqwest::Result<reqwest::Response> {
    let url = format!("{}/api/transaction-info?hash={}", api_url, txid);
    let cli = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(timeout as u64))
        .user_agent(utils::user_agent::get())
        .build()?;
    cli.get(&url).send().await
}

pub async fn get_transaction_info<T: DeserializeOwned>(
    txid: &str,
    api_url: &str,
    timeout: u8,
) -> Result<T> {
    let res = _get_transaction_info(txid, api_url, timeout)
        .await?
        .json::<T>()
        .await?;
    Ok(res)
}

pub async fn get_usdt_tran_info(
    txid: &str,
    api_url: &str,
    timeout: u8,
) -> Result<TransactionTRC20> {
    get_transaction_info(txid, api_url, timeout).await
}

pub async fn get_trx_transaction_info(
    txid: &str,
    api_url: &str,
    timeout: u8,
) -> Result<TransactionTRX> {
    get_transaction_info(txid, api_url, timeout).await
}

pub async fn usdt_tran(cfg: &config::TronConfig, txid: &str) -> Result<TransactionTRC20> {
    get_usdt_tran_info(txid, &cfg.api_url, cfg.fetch_timeout).await
}

pub async fn trx_tran(cfg: &config::TronConfig, txid: &str) -> Result<TransactionTRX> {
    get_trx_transaction_info(txid, &cfg.api_url, cfg.fetch_timeout).await
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_tron_get_usdt_transaction() {
        let info = super::get_usdt_tran_info(
            "4799e4960a90c6e2526f23ad2c5326f9e6a27f5995f0c6b1912be710fb167a32",
            "https://nileapi.tronscan.org",
            10,
        )
        .await;
        println!("{:?}", info);
    }
    #[tokio::test]
    async fn test_tron_get_trx_transaction() {
        let info = super::get_trx_transaction_info(
            "f430e1f2bc63807319c0512b53405abb314c301e2b634509f2fdc1b1c35bca96",
            "https://nileapi.tronscan.org",
            10,
        )
        .await;
        println!("{:?}", info);
    }
}
