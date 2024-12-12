use std::sync::Arc;

use crate::utils::config::DeepBookConfig;

pub struct FlashLoanContract {
    config: Arc<DeepBookConfig>,
}

impl FlashLoanContract {
    pub fn new(config: Arc<DeepBookConfig>) -> Self {
        Self { config }
    }
}