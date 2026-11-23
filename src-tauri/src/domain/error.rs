use std::{
    error::Error,
    fmt::{Debug, Display},
};

use serde::Serialize;

pub type IResult<T> = std::result::Result<T, IError>;
pub type CResult<T> = std::result::Result<T, CommandError>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandError {
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug)]
pub enum IError {
    Unknow(String),                         // 10000
    Settings(String),                       // 10100
    VrcOscApi(String),                      // 10200
    VrcOscClientLock(String),               // 10201
    VrcOscClientInit(String),               // 10202
    FileOperate(String),                    // 10300
    Chatbox(String),                        // 10400
    DynamicVariablePArgumentsError(String), // 10401
    Avatar(String),                         // 10500
    SystemInfo(String),                     // 10600
    Tauri(String),                          // 11000
}

impl From<IError> for CommandError {
    fn from(value: IError) -> Self {
        let (code, message) = match value {
            IError::Unknow(msg) => ("10000", msg),
            IError::Settings(message) => ("10100", message),
            IError::VrcOscApi(msg) => ("10200", msg),
            IError::VrcOscClientLock(msg) => ("10201", msg),
            IError::VrcOscClientInit(msg) => ("10202", msg),
            IError::Tauri(msg) => ("11000", msg),
            IError::FileOperate(msg) => ("10300", msg),
            IError::Chatbox(msg) => ("10400", msg),
            IError::DynamicVariablePArgumentsError(msg) => ("10401", msg),
            IError::Avatar(msg) => ("10500", msg),
            IError::SystemInfo(msg) => ("10600", msg),
        };
        Self { code, message }
    }
}

impl Display for IError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for IError {}
