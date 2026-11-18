use crate::{
    adapters,
    domain::error::{IError, IResult},
};

const HEIGHT_VALUES_FILE_NAME: &str = "avatar-height-values.json";
const MIN_HEIGHT: f32 = 0.01;
const MAX_HEIGHT: f32 = 100.0;

pub fn change_avatar_height(height: f32) -> IResult<()> {
    validate_height(height)?;
    let client = adapters::vrc_osc_api::get_client()?;
    client
        .write()
        .map_err(|e| IError::VrcOscClientLock(format!("lock error={e:?}")))?
        .send(vrc_osc_api::vrchat::avatar::AvatarEyeheight::new(height))
        .map(|_| ())
        .map_err(|e| IError::VrcOscApi(format!("Send message failed,err={e:?}")))
}

pub fn get_avatar_height_values() -> IResult<Vec<f32>> {
    let contents = adapters::file_system::read_config_file(HEIGHT_VALUES_FILE_NAME)?;
    if contents.is_empty() {
        return Ok(Vec::new());
    }

    let values: Vec<f32> = serde_json::from_slice(&contents).map_err(|error| {
        IError::Avatar(format!("Could not parse saved avatar heights: {error}"))
    })?;
    normalize_height_values(values)
}

pub fn save_avatar_height_values(values: Vec<f32>) -> IResult<Vec<f32>> {
    let values = normalize_height_values(values)?;
    let contents = serde_json::to_vec_pretty(&values)
        .map_err(|error| IError::Avatar(format!("Could not serialize avatar heights: {error}")))?;
    adapters::file_system::write_config_file(HEIGHT_VALUES_FILE_NAME, contents)?;
    Ok(values)
}

fn validate_height(height: f32) -> IResult<()> {
    if !height.is_finite() || !(MIN_HEIGHT..=MAX_HEIGHT).contains(&height) {
        return Err(IError::Avatar(format!(
            "Avatar height must be a finite number from {MIN_HEIGHT} to {MAX_HEIGHT}."
        )));
    }
    Ok(())
}

fn normalize_height_values(values: Vec<f32>) -> IResult<Vec<f32>> {
    let mut unique_values = Vec::with_capacity(values.len());
    for height in values {
        validate_height(height)?;
        if !unique_values.contains(&height) {
            unique_values.push(height);
        }
    }
    unique_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    Ok(unique_values)
}

#[cfg(test)]
mod tests {
    use super::{normalize_height_values, validate_height};

    #[test]
    fn validates_height_range_and_finite_values() {
        assert!(validate_height(0.01).is_ok());
        assert!(validate_height(100.0).is_ok());
        assert!(validate_height(0.0).is_err());
        assert!(validate_height(100.01).is_err());
        assert!(validate_height(f32::NAN).is_err());
        assert!(validate_height(f32::INFINITY).is_err());
    }

    #[test]
    fn removes_duplicate_shortcuts_without_reordering() {
        assert_eq!(
            normalize_height_values(vec![25.0, 50.5, 25.0]).unwrap(),
            vec![25.0, 50.5]
        );
    }

    #[test]
    fn rejects_invalid_shortcut_values() {
        assert!(normalize_height_values(vec![25.0, f32::NAN]).is_err());
    }
}
