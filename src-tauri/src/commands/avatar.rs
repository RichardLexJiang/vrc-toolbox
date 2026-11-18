use crate::{application, domain::error::CResult};

#[tauri::command]
pub async fn change_avatar_height(height: f32) -> CResult<()> {
    application::avatar::change_avatar_height(height).map_err(Into::into)
}

#[tauri::command]
pub fn get_avatar_height_values() -> CResult<Vec<f32>> {
    application::avatar::get_avatar_height_values().map_err(Into::into)
}

#[tauri::command]
pub fn save_avatar_height_values(height_values: Vec<f32>) -> CResult<Vec<f32>> {
    application::avatar::save_avatar_height_values(height_values).map_err(Into::into)
}
