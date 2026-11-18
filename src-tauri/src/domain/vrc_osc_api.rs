use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatus {
    status: Status,
}

impl ConnectionStatus {
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn status(&self) -> Status {
        self.status
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    #[default]
    Unknown,
    Connected,
    NoConnection,
}
