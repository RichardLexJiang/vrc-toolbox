use std::{
    error::Error,
    sync::{Arc, Mutex, OnceLock},
};

use sysinfo::System;

use crate::domain::error::{IError, IResult};

static SYS: OnceLock<Option<Arc<Mutex<System>>>> = OnceLock::new();

pub fn get_system() -> IResult<Arc<Mutex<System>>> {
    SYS.get_or_init(|| match System::new() {
        Ok(s) => Some(Arc::new(Mutex::new(s))),
        Err(_) => None,
    })
    .clone()
    .ok_or_else(|| IError::SystemInfo(format!("failed to fetch system info")))
}

pub fn get_system_info<F, T>(f: F) -> IResult<T>
where
    F: Fn(std::sync::MutexGuard<'_, System>) -> IResult<T>,
{
    let lock = get_system()?;
    f(lock.lock().map_err(|e| lock_error(e))?)
}

fn lock_error(e: impl Error) -> IError {
    IError::SystemInfo(format!("failed to fetch system info lock:{e}"))
}
