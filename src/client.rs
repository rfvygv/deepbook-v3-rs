use std::{collections::HashMap, sync::Arc};

use deepbook_v3_rs::{
    error::DeepBookResult,
    transactions::{
        balance_manager::BalanceManagerContract, deepbook::DeepBookContract,
        deepbook_admin::DeepBookAdminContract, flash_loans::FlashLoanContract,
        governance::GovernanceContract,
    },
    types::index::{BalanceManager, Environment},
    utils::{
        config::DeepBookConfig,
        constants::{CoinMap, PoolMap},
    },
};
use sui_sdk::SuiClient;

pub struct DeepBookClient {
    pub client: SuiClient,
    config: DeepBookConfig,
    address: String,
    pub balance_manager: BalanceManagerContract,
    pub deep_book: DeepBookContract,
    pub deep_book_admin: DeepBookAdminContract,
    pub flash_loans: FlashLoanContract,
    pub governance: GovernanceContract,
}

impl DeepBookClient {
    pub fn new(
        client: SuiClient,
        address: &str,
        env: Environment,
        balance_managers: Option<HashMap<String, BalanceManager>>,
        coins: Option<CoinMap>,
        pools: Option<PoolMap>,
        admin_cap: Option<String>,
    ) -> DeepBookResult<Self> {
        let config = DeepBookConfig::new(env, address, admin_cap, balance_managers, coins, pools)?;

        Ok(Self {
            client,
            address: address.to_string(),
            config: config.clone(),
            balance_manager: BalanceManagerContract::new(Arc::new(config.clone())),
            deep_book: DeepBookContract::new(Arc::new(config.clone())),
            deep_book_admin: DeepBookAdminContract::new(Arc::new(config.clone())),
            flash_loans: FlashLoanContract::new(Arc::new(config.clone())),
            governance: GovernanceContract::new(Arc::new(config.clone())),
        })
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
