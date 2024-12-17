use std::sync::Arc;

use crate::utils::config::DeepBookConfig;

#[derive(Clone)]
pub struct GovernanceContract {
    config: Arc<DeepBookConfig>,
}

impl GovernanceContract {
    pub fn new(config: Arc<DeepBookConfig>) -> Self {
        Self { config }
    }
}