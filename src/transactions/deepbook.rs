use std::sync::Arc;

use crate::utils::config::DeepBookConfig;

pub struct DeepBookContract {
    config: Arc<DeepBookConfig>,
}

impl DeepBookContract {
    pub fn new(config: Arc<DeepBookConfig>) -> Self {
        Self { config }
    }
}