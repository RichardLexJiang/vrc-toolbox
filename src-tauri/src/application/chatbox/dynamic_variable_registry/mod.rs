use std::{
    error::Error,
    sync::{Arc, Mutex, OnceLock},
};

use crate::{
    application::chatbox::dynamic_variable_registry::{
        provider::DynamicVariableProvider,
        registry::DynamicVariableRegistry,
        system_info_provider::{
            CpuPercentUsageProvider, MemoryPercentUsageProvider, TotalRamMemoryProvider,
            UsedRamMemoryProvider,
        },
        time_info_provider::LocalTimeProvider,
    },
    domain::{
        chatbox::{DynamicVariableDescription, RenderContext},
        error::{IError, IResult},
    },
};

pub(crate) mod provider;
pub(crate) mod registry;
pub(crate) mod system_info_provider;
pub(crate) mod time_info_provider;

static GLOBAL_DYNAMIC_VARIABLE_REGISTRY: OnceLock<Arc<Mutex<DynamicVariableRegistry>>> =
    OnceLock::new();

pub fn provide_value(key: &str, ctx: RenderContext) -> IResult<String> {
    global_dynamic_variable_registry()
        .lock()
        .map_err(lock_error)?
        .provide_value(key, ctx)
}

pub fn supported_dynamic_var_list() -> IResult<Vec<DynamicVariableDescription>> {
    global_dynamic_variable_registry()
        .lock()
        .map(|r| r.supported_dynamic_var_list())
        .map_err(lock_error)
}

pub fn global_dynamic_variable_registry() -> Arc<Mutex<DynamicVariableRegistry>> {
    GLOBAL_DYNAMIC_VARIABLE_REGISTRY
        .get_or_init(|| Arc::new(Mutex::new(initial_global_register())))
        .clone()
}

fn initial_global_register() -> DynamicVariableRegistry {
    let list: Vec<Box<dyn DynamicVariableProvider + Send>> = vec![
        Box::new(CpuPercentUsageProvider),
        Box::new(MemoryPercentUsageProvider),
        Box::new(UsedRamMemoryProvider),
        Box::new(TotalRamMemoryProvider),
        Box::new(LocalTimeProvider),
    ];

    let mut x = DynamicVariableRegistry::new();
    for ele in list {
        x.insert_provider(ele);
    }
    x
}

fn lock_error(e: impl Error) -> IError {
    IError::DynamicVariableRegistryLockFailed(format!("Lock error={e:?}"))
}
