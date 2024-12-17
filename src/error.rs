use move_core_types::account_address::AccountAddressParseError;
use sui_types::base_types::ObjectIDParseError;

pub type DeepBookResult<T> = Result<T, DeepBookError>;

#[derive(thiserror::Error, Debug)]
pub enum DeepBookError {
    #[error(transparent)]
    AnyhowError(anyhow::Error),

    #[error(transparent)]
    AccountAddressParseError(AccountAddressParseError),

    #[error(transparent)]
    ObjectIDParseError(ObjectIDParseError),

    #[error(transparent)]
    SuiSDKError(sui_sdk::error::Error),
}
