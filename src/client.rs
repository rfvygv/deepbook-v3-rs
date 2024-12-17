use anyhow::anyhow;
use shared_crypto::intent::Intent;
use std::{collections::HashMap, sync::Arc};
use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_types::transaction::ProgrammableTransaction;
// use sui_config::{sui_config_dir, SUI_KEYSTORE_FILENAME};
use crate::error::{DeepBookError, DeepBookResult};
use sui_sdk::SuiClient;
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::{
    base_types::SuiAddress,
    transaction::{Transaction, TransactionData},
};

use crate::types::index::{BalanceManager, Environment};
use crate::utils::{
    config::DeepBookConfig,
    constants::{CoinMap, PoolMap},
};

use crate::transactions::{
    balance_manager::BalanceManagerContract, deepbook::DeepBookContract,
    deepbook_admin::DeepBookAdminContract, flash_loans::FlashLoanContract,
    governance::GovernanceContract,
};

#[derive(Clone)]
pub struct DeepBookClient {
    pub client: Arc<SuiClient>,
    config: DeepBookConfig,
    address: SuiAddress,
    pub balance_manager: BalanceManagerContract,
    pub deep_book: DeepBookContract,
    pub deep_book_admin: DeepBookAdminContract,
    pub flash_loans: FlashLoanContract,
    pub governance: GovernanceContract,
}

impl DeepBookClient {
    pub fn new(
        client: Arc<SuiClient>,
        address: SuiAddress,
        env: Environment,
        balance_managers: Option<HashMap<String, BalanceManager>>,
        coins: Option<CoinMap>,
        pools: Option<PoolMap>,
        admin_cap: Option<String>,
    ) -> DeepBookResult<Self> {
        let config = DeepBookConfig::new(env, address, admin_cap, balance_managers, coins, pools)?;

        Ok(Self {
            client,
            address,
            config: config.clone(),
            balance_manager: BalanceManagerContract::new(Arc::new(config.clone())),
            deep_book: DeepBookContract::new(Arc::new(config.clone())),
            deep_book_admin: DeepBookAdminContract::new(Arc::new(config.clone())),
            flash_loans: FlashLoanContract::new(Arc::new(config.clone())),
            governance: GovernanceContract::new(Arc::new(config.clone())),
        })
    }

    pub async fn execute_and_sign_transaction(
        &self,
        pt: ProgrammableTransaction,
        gas_budget: u64,
    ) -> DeepBookResult<sui_sdk::rpc_types::SuiTransactionBlockResponse> {
        let gas_price = self
            .client
            .read_api()
            .get_reference_gas_price()
            .await
            .map_err(DeepBookError::SuiSDKError)?;

        let coins = self
            .client
            .coin_read_api()
            .get_coins(self.address, None, None, None)
            .await
            .map_err(DeepBookError::SuiSDKError)?;

        let coin = coins
            .data
            .into_iter()
            .next()
            .ok_or(DeepBookError::AnyhowError(anyhow!("Coin not found")))?;

        let tx_data = TransactionData::new_programmable(
            self.address,
            vec![coin.object_ref()],
            pt,
            gas_budget,
            gas_price,
        );

        let keystore = FileBasedKeystore::new(
            &sui_config_dir()
                .map_err(DeepBookError::AnyhowError)?
                .join(SUI_KEYSTORE_FILENAME),
        )
        .map_err(|e| DeepBookError::AnyhowError(anyhow!(e)))?;

        let signature = keystore
            .sign_secure(&self.address, &tx_data, Intent::sui_transaction())
            .map_err(|e| DeepBookError::AnyhowError(anyhow!("Signature error: {}", e)))?;

        let transaction_response: sui_sdk::rpc_types::SuiTransactionBlockResponse = self
            .client
            .quorum_driver_api()
            .execute_transaction_block(
                Transaction::from_data(tx_data, vec![signature]),
                SuiTransactionBlockResponseOptions::full_content(),
                Some(ExecuteTransactionRequestType::WaitForLocalExecution),
            )
            .await
            .map_err(DeepBookError::SuiSDKError)?;

        Ok(transaction_response)
    }

    pub async fn check_manager_balance() {}

    pub async fn whitelisted() {}

    pub async fn get_quote_quantity_out() {}

    pub async fn get_base_quantity_out() {}

    pub async fn get_quantity_out() {}

    pub async fn account_open_orders() {}

    pub async fn get_order() {}

    pub async fn get_order_normalized() {}

    pub async fn get_orders() {}

    pub async fn get_level2_range() {}

    pub async fn get_level2_ticks_from_mid() {}

    pub async fn vault_balances() {}

    pub async fn get_pool_id_by_assets() {}

    pub async fn mid_price() {}

    pub async fn pool_trade_params() {}

    pub async fn pool_book_params() {}

    pub async fn account() {}

    pub async fn locked_balance() {}

    pub async fn get_pool_deep_price() {}

    pub async fn decode_order_id() {}
}
