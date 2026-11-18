use crate::{
    application,
    domain::{
        chatbox::{
            ChatboxHistoryDto, ChatboxTemplateDto, DynamicVariableDescription, RepeatStatus,
        },
        error::CResult,
    },
};

#[tauri::command]
pub async fn send_message(_: tauri::AppHandle, message: String, append_his: bool) -> CResult<()> {
    application::chatbox::message::send_message(message, append_his).map_err(Into::into)
}

#[tauri::command]
pub async fn get_chatbox_history() -> CResult<Vec<ChatboxHistoryDto>> {
    application::chatbox::message::read_history().map_err(Into::into)
}

#[tauri::command]
pub async fn get_template_list() -> CResult<Vec<ChatboxTemplateDto>> {
    application::chatbox::repeator::get_template_list().map_err(Into::into)
}

#[tauri::command]
pub async fn write_template_list(list: Vec<ChatboxTemplateDto>) -> CResult<()> {
    application::chatbox::repeator::write_template_list(list).map_err(Into::into)
}

#[tauri::command]
pub async fn start_chatbox_repeat(
    app: tauri::AppHandle,
    message: Vec<String>,
) -> CResult<RepeatStatus> {
    application::chatbox::repeator::start_repeating(app, message).map_err(Into::into)
}

#[tauri::command]
pub async fn stop_chatbox_repeat() -> CResult<RepeatStatus> {
    application::chatbox::repeator::stop_repeating().map_err(Into::into)
}

#[tauri::command]
pub async fn get_chatbox_repeat_status() -> CResult<RepeatStatus> {
    application::chatbox::repeator::get_repeat_status().map_err(Into::into)
}

#[tauri::command]
pub async fn supported_dynamic_var_list() -> CResult<Vec<DynamicVariableDescription>> {
    Ok(application::chatbox::dynamic_variable::supported_dynamic_var_list())
}
