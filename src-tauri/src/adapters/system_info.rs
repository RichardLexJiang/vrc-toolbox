use std::sync::{Arc, OnceLock};

use sysinfo::System;

use crate::domain::error::{IError, IResult};

static SYS: OnceLock<Option<Arc<System>>> = OnceLock::new();

pub fn get_system_info() -> IResult<Arc<System>> {
    SYS.get_or_init(|| match System::new() {
        Ok(s) => Some(Arc::new(s)),
        Err(_) => None,
    })
    .clone()
    .ok_or_else(system_info_error)
}

pub fn get_global_cpu_usage() -> IResult<f32> {
    get_system_info().map(|s| s.global_cpu_usage())
}

pub fn get_memory_percent_usage() -> IResult<f64> {
    get_system_info().map(|s| s.used_memory() as f64 / s.total_memory() as f64)
}

fn system_info_error() -> IError {
    IError::SystemInfo(format!("failed to fetch system info"))
}
