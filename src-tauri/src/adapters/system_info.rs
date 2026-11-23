use std::{
    error::Error,
    sync::{Arc, Mutex, OnceLock},
};

use sysinfo::System;

use crate::domain::error::{IError, IResult};

static SYS: OnceLock<Option<Arc<Mutex<System>>>> = OnceLock::new();

pub fn get_system_info() -> IResult<Arc<Mutex<System>>> {
    SYS.get_or_init(|| match System::new() {
        Ok(s) => Some(Arc::new(Mutex::new(s))),
        Err(_) => None,
    })
    .clone()
    .ok_or_else(|| IError::SystemInfo(format!("failed to fetch system info")))
}

pub fn get_global_cpu_usage() -> IResult<f32> {
    let lock = get_system_info()?;
    let mut s = lock.lock().map_err(|e| lock_error(e))?;
    s.refresh_cpu_usage();
    Ok(s.global_cpu_usage())
}

pub fn get_memory_percent_usage() -> IResult<f64> {
    let lock = get_system_info()?;
    let mut s = lock.lock().map_err(|e| lock_error(e))?;
    s.refresh_memory();
    Ok((s.used_memory() as f64) / (s.total_memory() as f64))
}

pub fn get_total_memory() -> IResult<u64> {
    let lock = get_system_info()?;
    let mut s = lock.lock().map_err(|e| lock_error(e))?;
    s.refresh_memory();
    Ok(s.total_memory())
}

pub fn get_used_memory() -> IResult<u64> {
    let lock = get_system_info()?;
    let mut s = lock.lock().map_err(|e| lock_error(e))?;
    s.refresh_memory();
    Ok(s.used_memory())
}

fn lock_error(e: impl Error) -> IError {
    IError::SystemInfo(format!("failed to fetch system info lock:{e}"))
}
