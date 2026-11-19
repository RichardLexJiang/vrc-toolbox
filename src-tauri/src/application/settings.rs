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
    parse_settings(&file_system::read_config_file(SETTINGS_FILE_NAME)?)
}

fn parse_settings(contents: &[u8]) -> IResult<SettingsDto> {
    if contents.is_empty() {
        return Ok(SettingsDto::default());
    }

    serde_json::from_slice(contents)
        .map_err(|error| IError::Settings(format!("Could not parse settings err: {error}",)))
}

pub fn write_settings(settings: &SettingsDto) -> IResult<()> {
    let contents = serde_json::to_vec_pretty(settings)
        .map_err(|error| IError::Settings(format!("Could not serialize settings: {error}")))?;

    file_system::write_config_file(SETTINGS_FILE_NAME, contents)
}

#[cfg(test)]
mod tests {
    use super::parse_settings;
    use crate::domain::settings::SettingsDto;

    #[test]
    fn empty_settings_use_defaults() {
        let settings = parse_settings(&[]).expect("empty settings should use defaults");

        assert_eq!(settings.osc_target(), SettingsDto::default().osc_target());
    }

    #[test]
    fn malformed_settings_return_an_error() {
        assert!(parse_settings(b"{").is_err());
    }
}
