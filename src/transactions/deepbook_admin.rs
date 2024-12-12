use std::sync::Arc;

use crate::utils::config::DeepBookConfig;

pub struct DeepBookAdminContract {
    config: Arc<DeepBookConfig>,
}

impl DeepBookAdminContract {
    pub fn new(config: Arc<DeepBookConfig>) -> Self {
        Self { config }
    }
}
