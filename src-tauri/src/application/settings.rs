use crate::{
    adapters::file_system,
    adapters::vrc_osc_api::reset_target,
    domain::{
        error::{IError, IResult},
        settings::SettingsDto,
    },
};

const SETTINGS_FILE_NAME: &str = "settings.json";

pub fn apply_settings(settings: &SettingsDto) -> IResult<()> {
    if let Ok(prev) = read_settings() {
        // check specific setting changing
        if !prev
            .osc_target()
            .eq_ignore_ascii_case(settings.osc_target())
        {
            // also try to change client
            reset_target(settings.osc_target())?;
        }
    }
    write_settings(settings)
}

pub fn read_settings() -> IResult<SettingsDto> {
    serde_json::from_slice(&file_system::read_config_file(SETTINGS_FILE_NAME)?)
        .map_err(|error| IError::Settings(format!("Could not parse settings err: {error}",)))
}

pub fn write_settings(settings: &SettingsDto) -> IResult<()> {
    let contents = serde_json::to_vec_pretty(settings)
        .map_err(|error| IError::Settings(format!("Could not serialize settings: {error}")))?;

    file_system::write_config_file(SETTINGS_FILE_NAME, contents)
}
