use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use crate::domain::error::{IError, IResult};

static CONFIG_DIR: OnceLock<Arc<PathBuf>> = OnceLock::new();

pub fn init_config_dir(config_dir: &Path) -> IResult<()> {
    CONFIG_DIR.get_or_init(|| Arc::new(config_dir.to_path_buf()));
    Ok(())
}

pub fn write_config_file(file_name: &str, contents: impl AsRef<[u8]>) -> IResult<()> {
    let config_dir = CONFIG_DIR
        .get()
        .ok_or_else(|| IError::FileOperate(format!("Initial config dir doesn't exist")))?
        .clone();

    fs::create_dir_all(config_dir.as_path()).map_err(|error| {
        IError::FileOperate(format!(
            "Could not create config directory {config_dir:?}: {error}"
        ))
    })?;

    fs::write(&config_dir.join(file_name), contents).map_err(|error| {
        IError::FileOperate(format!(
            "Could not write settings at {}: {error}",
            config_dir.display()
        ))
    })
}

pub fn read_config_file(file_name: &str) -> IResult<Vec<u8>> {
    let config_dir = CONFIG_DIR
        .get()
        .ok_or_else(|| IError::FileOperate(format!("Initial config dir doesn't exist")))?
        .join(file_name);
    match fs::read(config_dir.as_path()) {
        Ok(contents) => Ok(contents),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(vec![]),
        Err(error) => Err(IError::FileOperate(format!(
            "Could not read settings at {}: {error}",
            config_dir.display()
        ))),
    }
}
