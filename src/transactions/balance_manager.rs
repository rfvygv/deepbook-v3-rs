use crate::utils::config::DeepBookConfig;
use std::str::FromStr;
use std::sync::Arc;
use sui_types::{base_types::ObjectID, transaction::ProgrammableTransaction};

use crate::error::{DeepBookError, DeepBookResult};
use move_core_types::account_address::AccountAddress;
use sui_sdk::types::{
    programmable_transaction_builder::ProgrammableTransactionBuilder, transaction::Command,
};
use sui_types::transaction::ProgrammableMoveCall;
use sui_types::type_input::{StructInput, TypeInput};

#[derive(Debug, Clone)]
pub struct BalanceManagerContract {
    config: Arc<DeepBookConfig>,
}

impl BalanceManagerContract {
    pub fn new(config: Arc<DeepBookConfig>) -> Self {
        Self { config }
    }

    pub fn get_deepbook_package_object_id(&self) -> DeepBookResult<ObjectID> {
        let object_id =
            ObjectID::from_hex_literal(self.config.deepbook_package_id.clone().as_str())
                .map_err(DeepBookError::ObjectIDParseError)?;
        Ok(object_id)
    }

    pub fn create_and_share_balance_manager(&self) -> DeepBookResult<ProgrammableTransaction> {
        let mut ptb = ProgrammableTransactionBuilder::new();
        let manager_object_id = self.get_deepbook_package_object_id()?;

        let manager = ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: manager_object_id,
            module: "balance_manager".to_string(),
            function: "new".to_string(),
            type_arguments: vec![],
            arguments: vec![],
        })));

        let share_object_type_argument = TypeInput::Struct(Box::new(StructInput {
            address: AccountAddress::from_hex_literal(
                self.config.deepbook_package_id.clone().as_str(),
            )
            .map_err(DeepBookError::AccountAddressParseError)?,
            module: "balance_manager".to_string(),
            name: "BalanceManager".to_string(),
            type_params: vec![],
        }));

        ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
            package: ObjectID::from_str("0x2").map_err(DeepBookError::ObjectIDParseError)?,
            module: "transfer".to_string(),
            function: "public_share_object".to_string(),
            type_arguments: vec![share_object_type_argument],
            arguments: vec![manager],
        })));

        let builder = ptb.finish();

        Ok(builder)
    }

    pub async fn deposit_into_manager() {}

    pub async fn withdraw_from_manager() {}

    pub async fn withdraw_all_from_manager() {}

    pub fn check_manager_balance() {}

    pub async fn generate_proof() {}
}
