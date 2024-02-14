use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigSettings {
    pub settings: ConfigSettingsEntity,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigSettingsEntity {
    pub enable_logs_to_file: bool,
    pub radio_url: String,
}
