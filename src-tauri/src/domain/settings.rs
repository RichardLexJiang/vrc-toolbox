use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsDto {
    osc_target: String,
}

impl SettingsDto {
    pub fn osc_target(&self) -> &str {
        &self.osc_target
    }
}

impl Default for SettingsDto {
    fn default() -> Self {
        Self {
            osc_target: "127.0.0.1:9000".to_string(),
        }
    }
}
