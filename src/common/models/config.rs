use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigSettings {
    pub settings: ConfigSettingsEntity,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigSettingsEntity {
    pub enable_logs_to_file: bool,
    pub radio_url: String,
    pub radio_streaming_website_url: String,
    pub google_api_key: String,
    pub google_default_client_id: String,
    pub google_default_client_secret: String,
}
