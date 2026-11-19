use chrono::Local;

use crate::{
    adapters,
    application::chatbox::dynamic_variable::DynamicVariable::CpuUsage,
    domain::{chatbox::DynamicVariableDescription, error::IResult},
};

pub fn supported_dynamic_var_list() -> Vec<DynamicVariableDescription> {
    DynamicVariable::SUPPORTED
        .into_iter()
        .map(|variable| {
            DynamicVariableDescription::new(
                format!("${{{}}}", variable.key()),
                variable.example_value(),
                variable.description(),
            )
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicVariable {
    CpuUsage,
    MemoryPercentUsage,
    LocalTime(&'static str),
}

impl DynamicVariable {
    const SUPPORTED: [Self; 3] = [
        Self::LocalTime("%H:%M"),
        Self::CpuUsage,
        Self::MemoryPercentUsage,
    ];

    pub fn key(&self) -> &str {
        match self {
            CpuUsage => "cpu_usage",
            DynamicVariable::LocalTime(_) => "local_time",
            DynamicVariable::MemoryPercentUsage => "memory_usage",
        }
    }

    pub fn generate_value(&self) -> IResult<String> {
        match self {
            CpuUsage => adapters::system_info::get_global_cpu_usage().map(|v| format!("{:.0}", v)),
            DynamicVariable::LocalTime(f) => Ok(Local::now().format(f).to_string()),
            DynamicVariable::MemoryPercentUsage => {
                adapters::system_info::get_memory_percent_usage().map(|v| format!("{:.0}", v))
            }
        }
    }

    fn example_value(&self) -> &'static str {
        match self {
            Self::CpuUsage => "42%",
            Self::MemoryPercentUsage => "68%",
            Self::LocalTime(_) => "2026-11-15 20:56:30",
        }
    }

    fn description(&self) -> String {
        match self {
            Self::CpuUsage => format!(
                "{} Overall system CPU usage as a percentage.",
                Self::CpuUsage.key()
            ),
            Self::MemoryPercentUsage => {
                "Used physical memory as a percentage of total RAM.".to_string()
            }
            Self::LocalTime(_) => "Current local time in the format you want, example format: ${local_time:%Y-%m-%d %H:%M:%S}".to_string(),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test() {}
}
