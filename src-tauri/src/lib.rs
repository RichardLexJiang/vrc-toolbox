mod adapters;
mod application;
mod commands;
mod domain;

use std::sync::Mutex;

use tauri::{App, Manager};

use crate::{
    adapters::file_system::init_config_dir,
    commands::{
        chatbox::{
            get_chatbox_history, get_chatbox_repeat_status, get_template_list, send_message,
            start_chatbox_repeat, stop_chatbox_repeat, supported_dynamic_var_list,
            write_template_list,
        },
        settings::{get_connection_status, get_local_settings, save_local_settings},
    },
    domain::{
        error::{IError, IResult},
        vrc_osc_api::ConnectionStatus,
    },
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            send_message,
            get_connection_status,
            get_local_settings,
            save_local_settings,
            get_chatbox_history,
            get_template_list,
            write_template_list,
            start_chatbox_repeat,
            stop_chatbox_repeat,
            get_chatbox_repeat_status,
            supported_dynamic_var_list,
            commands::avatar::change_avatar_height,
            commands::avatar::get_avatar_height_values,
            commands::avatar::save_avatar_height_values,
        ])
        .manage(Mutex::new(ConnectionStatus::default()))
        .setup(|app| Ok(init(app).map_err(Box::new)?))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn init(app: &mut App) -> IResult<()> {
    // init config dir
    init_config_dir(
        app.app_handle()
            .path()
            .app_config_dir()
            .map_err(|e| IError::Tauri(format!("Cannot get tauri app_config_dir, err={e:?}")))?
            .as_path(),
    )?;
    // init client
    let settings = application::settings::read_settings()?;
    let status = if adapters::vrc_osc_api::init_client(settings.osc_target()).is_ok() {
        domain::vrc_osc_api::Status::Connected
    } else {
        domain::vrc_osc_api::Status::NoConnection
    };
    app.state::<Mutex<ConnectionStatus>>()
        .lock()
        .map_err(|error| domain::error::IError::VrcOscClientLock(format!("lock error={error:?}")))?
        .set_status(status);
    Ok(())
}
