use std::sync::Arc;

use crate::utils::config::DeepBookConfig;
use sui_transaction_builder::TransactionBuilder;

#[derive(Debug, Clone)]
pub struct BalanceManagerContract {
    config: Arc<DeepBookConfig>,
}

impl BalanceManagerContract {
    pub fn new(config: Arc<DeepBookConfig>) -> Self {
        Self { config }
    }

    pub async fn create_and_share_balance_manager() {}

    pub async fn deposit_into_manager() {}

    pub async fn withdraw_from_manager() {}

    pub async fn withdraw_all_from_manager() {}

    pub async fn check_manager_balance() {}

    pub async fn generate_proof() {}
}
