use std::sync::Mutex;

use tauri::{AppHandle, State};

use crate::{
    application::settings::{apply_settings, read_settings},
    domain::{self, error::CResult, settings::SettingsDto},
};

#[tauri::command]
pub fn get_local_settings(_: AppHandle) -> CResult<SettingsDto> {
    read_settings().map_err(Into::into)
}

#[tauri::command]
pub fn save_local_settings(_: AppHandle, settings: SettingsDto) -> CResult<()> {
    apply_settings(&settings).map_err(Into::into)
}

#[tauri::command]
pub fn get_connection_status(
    status: State<'_, Mutex<domain::vrc_osc_api::ConnectionStatus>>,
) -> CResult<domain::vrc_osc_api::Status> {
    status
        .lock()
        .map(|status| status.status())
        .map_err(|error| {
            domain::error::IError::VrcOscClientLock(format!("lock error={error:?}")).into()
        })
}
