use crate::error::{DeepBookError, DeepBookResult};
use crate::transactions::balance_manager::BalanceManagerContract;
use crate::types::index::{BalanceManager, Coin, Environment, Pool};
use crate::utils::constants::{
    CoinMap, PoolMap, MAINNET_COINS, MAINNET_PACKAGE_IDS, MAINNET_POOLS, TESTNET_COINS,
    TESTNET_PACKAGE_IDS, TESTNET_POOLS,
};
use anyhow::anyhow;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use sui_types::base_types::SuiAddress;

const FLOAT_SCALAR: u64 = 1000000000;
const MAX_TIMESTAMP: u128 = 1_844_674_407_370_955_161;
const GAS_BUDGET: f64 = 500000000.0 * 0.5; // Adjust based on benchmarking
const DEEP_SCALAR: u64 = 1000000;

#[derive(Debug, Clone)]
pub struct DeepBookConfig {
    coins: CoinMap,
    pools: PoolMap,
    pub balance_managers: HashMap<String, BalanceManager>,
    pub address: String,
    pub deepbook_package_id: String,
    pub registry_id: String,
    pub deep_treasury_id: String,
    pub admin_cap: Option<String>,
    pub balance_manager: Option<BalanceManagerContract>,
}

impl DeepBookConfig {
    pub fn new(
        env: Environment,
        address: &str,
        admin_cap: Option<String>,
        balance_managers: Option<HashMap<String, BalanceManager>>,
        coins: Option<CoinMap>,
        pools: Option<PoolMap>,
    ) -> DeepBookResult<Self> {
        SuiAddress::from_str(address)
            .map_err(|err| DeepBookError::AnyhowError(anyhow!("Wrong address: {}", err)))?;

        let (deepbook_package_id, registry_id, deep_treasury_id) = match env {
            Environment::Mainnet => (
                MAINNET_PACKAGE_IDS.deepbook_package_id.clone(),
                MAINNET_PACKAGE_IDS.registry_id.clone(),
                MAINNET_PACKAGE_IDS.deep_treasury_id.clone(),
            ),
            Environment::Testnet => (
                TESTNET_PACKAGE_IDS.deepbook_package_id.clone(),
                TESTNET_PACKAGE_IDS.registry_id.clone(),
                TESTNET_PACKAGE_IDS.deep_treasury_id.clone(),
            ),
        };

        let config = Self {
            address: address.to_string(),
            admin_cap,
            balance_managers: balance_managers.unwrap_or_default(),
            coins: coins.unwrap_or_else(|| match env {
                Environment::Mainnet => MAINNET_COINS.clone(),
                Environment::Testnet => TESTNET_COINS.clone(),
            }),
            pools: pools.unwrap_or_else(|| match env {
                Environment::Mainnet => MAINNET_POOLS.clone(),
                Environment::Testnet => TESTNET_POOLS.clone(),
            }),
            deepbook_package_id,
            registry_id,
            deep_treasury_id,
            balance_manager: None,
        };

        let balance_manager = BalanceManagerContract::new(Arc::new(config.clone()));

        Ok(Self {
            balance_manager: Some(balance_manager),
            ..config
        })
    }

    pub fn get_coin(self, key: &str) -> DeepBookResult<Coin> {
        self.coins
            .get(key)
            .cloned()
            .ok_or_else(|| DeepBookError::AnyhowError(anyhow!("Coin not found for key: {}", key)))
    }

    pub fn get_pool(self, key: &str) -> DeepBookResult<Pool> {
        self.pools
            .get(key)
            .cloned()
            .ok_or_else(|| DeepBookError::AnyhowError(anyhow!("Pool not found for key: {}", key)))
    }

    pub fn get_balance_manager(self, manager_key: &str) -> DeepBookResult<BalanceManager> {
        self.balance_managers
            .get(manager_key)
            .cloned()
            .ok_or_else(|| {
                DeepBookError::AnyhowError(anyhow!(
                    "Balance manager with key {} not found.",
                    manager_key
                ))
            })
    }
}
