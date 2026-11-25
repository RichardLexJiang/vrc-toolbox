use crate::{
    adapters::system_info::get_system_info,
    application::chatbox::dynamic_variable_registry::provider::DynamicVariableProvider,
    domain::{chatbox::RenderContext, error::IResult},
};

pub struct CpuPercentUsageProvider;
impl DynamicVariableProvider for CpuPercentUsageProvider {
    fn key(&self) -> String {
        "cpu_usage".to_string()
    }

    fn example_value(&self) -> &str {
        "42%"
    }

    fn description(&self) -> String {
        format!("{} Overall system CPU usage as a percentage.", self.key())
    }

    fn evaluate(&self, _: &RenderContext) -> IResult<String> {
        get_system_info(|mut s| {
            s.refresh_cpu_usage();
            Ok(s.global_cpu_usage())
        })
        .map(|v| format!("{:.0}%", v))
    }
}

pub struct MemoryPercentUsageProvider;
impl DynamicVariableProvider for MemoryPercentUsageProvider {
    fn key(&self) -> String {
        "memory_usage".to_string()
    }

    fn example_value(&self) -> &str {
        "68%"
    }

    fn description(&self) -> String {
        "Used physical memory as a percentage of total RAM.".to_string()
    }

    fn evaluate(&self, _: &RenderContext) -> IResult<String> {
        get_system_info(|mut s| {
            s.refresh_memory();
            Ok((s.used_memory() as f64) / (s.total_memory() as f64))
        })
        .map(|v| format!("{:.0}%", v * 100.0))
    }
}

pub struct UsedRamMemoryProvider;
impl DynamicVariableProvider for UsedRamMemoryProvider {
    fn key(&self) -> String {
        "used_ram".to_string()
    }

    fn example_value(&self) -> &str {
        "16.5"
    }

    fn description(&self) -> String {
        "Currently used physical memory in GiB.".to_string()
    }

    fn evaluate(&self, _: &RenderContext) -> IResult<String> {
        get_system_info(|mut s| {
            s.refresh_memory();
            Ok(s.used_memory())
        })
        .map(|v| format!("{:.1}", (v as f64) / 1024.0 / 1024.0 / 1024.0))
    }
}

pub struct TotalRamMemoryProvider;
impl DynamicVariableProvider for TotalRamMemoryProvider {
    fn key(&self) -> String {
        "total_ram".to_string()
    }

    fn example_value(&self) -> &str {
        "32.1"
    }

    fn description(&self) -> String {
        "Total physical memory in GiB.".to_string()
    }

    fn evaluate(&self, _: &RenderContext) -> IResult<String> {
        get_system_info(|mut s| {
            s.refresh_memory();
            Ok(s.total_memory())
        })
        .map(|v| format!("{:.1}", (v as f64) / 1024.0 / 1024.0 / 1024.0))
    }
}
