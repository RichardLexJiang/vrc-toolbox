use chrono::Local;

use crate::{
    adapters,
    domain::{
        chatbox::{DynamicVariableDescription, PlayContext},
        error::{IError, IResult},
    },
};

pub fn supported_dynamic_var_list() -> Vec<DynamicVariableDescription> {
    [
        DynamicVariable::LocalTime("%H:%M".to_string()),
        DynamicVariable::CpuUsage,
        DynamicVariable::MemoryPercentUsage,
        DynamicVariable::Dots { count: 3 },
        DynamicVariable::TotalRamMemory,
        DynamicVariable::UsedRamMemory,
    ]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynamicVariable {
    CpuUsage,
    MemoryPercentUsage,
    TotalRamMemory,
    UsedRamMemory,
    LocalTime(String),
    Dots { count: usize },
}

impl DynamicVariable {
    pub fn key(&self) -> &str {
        match self {
            DynamicVariable::CpuUsage => "cpu_usage",
            DynamicVariable::LocalTime(_) => "local_time",
            DynamicVariable::MemoryPercentUsage => "memory_usage",
            DynamicVariable::Dots { .. } => "dots",
            DynamicVariable::TotalRamMemory => "total_ram",
            DynamicVariable::UsedRamMemory => "used_ram",
        }
    }

    pub fn new(key: &str, argument: Option<&str>) -> IResult<Option<Self>> {
        match key {
            "cpu_usage" => Ok(Some(DynamicVariable::CpuUsage)),
            "memory_usage" => Ok(Some(DynamicVariable::MemoryPercentUsage)),
            "dots" => {
                let count = match argument.map(str::trim).filter(|value| !value.is_empty()) {
                    None => 3,
                    Some(value) => value.parse::<usize>().map_err(|_| {
                        IError::DynamicVariablePArgumentsError(
                            "dots argument must be an integer from 1 to 6".to_string(),
                        )
                    })?,
                };
                let variable = DynamicVariable::Dots { count };
                variable.arg_check()?;
                Ok(Some(variable))
            }
            "local_time" => {
                let format = match argument.map(str::trim).filter(|value| !value.is_empty()) {
                    None => "%H:%M",
                    Some(value) => unquote_argument(value).unwrap_or(value),
                };
                Ok(Some(DynamicVariable::LocalTime(format.to_string())))
            }
            "used_ram" => Ok(Some(DynamicVariable::UsedRamMemory)),
            "total_ram" => Ok(Some(DynamicVariable::TotalRamMemory)),
            _ => Ok(None),
        }
    }

    pub fn evaluate(&self, ctx: &PlayContext) -> IResult<String> {
        match self {
            DynamicVariable::CpuUsage => {
                adapters::system_info::get_global_cpu_usage().map(|v| format!("{:.0}%", v))
            }
            DynamicVariable::LocalTime(f) => Ok(Local::now().format(f).to_string()),
            DynamicVariable::MemoryPercentUsage => {
                adapters::system_info::get_memory_percent_usage()
                    .map(|v| format!("{:.0}%", v * 100.0))
            }
            DynamicVariable::Dots { count } => Ok(".".repeat(if ctx.is_repeating() {
                (ctx.template_tick() % count) + 1
            } else {
                *count
            })),
            DynamicVariable::TotalRamMemory => adapters::system_info::get_total_memory()
                .map(|v| format!("{:.1}", (v as f64) / 1024.0 / 1024.0 / 1024.0)),
            DynamicVariable::UsedRamMemory => adapters::system_info::get_used_memory()
                .map(|v| format!("{:.1}", (v as f64) / 1024.0 / 1024.0 / 1024.0)),
        }
    }

    pub fn arg_check(&self) -> IResult<()> {
        match self {
            DynamicVariable::Dots { count } => {
                if count > &0usize && count <= &6usize {
                    Ok(())
                } else {
                    Err(IError::DynamicVariablePArgumentsError(format!("")))
                }
            }
            _ => Ok(()),
        }
    }

    fn example_value(&self) -> &'static str {
        match self {
            DynamicVariable::CpuUsage => "42%",
            DynamicVariable::MemoryPercentUsage => "68%",
            DynamicVariable::LocalTime(_) => "20:56",
            DynamicVariable::Dots { .. } => "...",
            DynamicVariable::TotalRamMemory => "32.1",
            DynamicVariable::UsedRamMemory => "16.5",
        }
    }

    fn description(&self) -> String {
        match self {
            DynamicVariable::CpuUsage => {
                format!("{} Overall system CPU usage as a percentage.", self.key())
            }
            DynamicVariable::MemoryPercentUsage => {
                "Used physical memory as a percentage of total RAM.".to_string()
            }
            DynamicVariable::LocalTime(_) => format!(
                "Current local time in the format you want, example format: ${{{}(\"%Y-%m-%d %H:%M:%S\")}}",
                self.key()
            ),
            DynamicVariable::Dots { .. } => format!(
                "${{{}(6)}} generate \"......\", number means the number of dots, min 1,max 6",
                self.key()
            ),
            DynamicVariable::TotalRamMemory => "Total physical memory in GiB.".to_string(),
            DynamicVariable::UsedRamMemory => {
                "Currently used physical memory in GiB.".to_string()
            }
        }
    }
}

fn unquote_argument(argument: &str) -> Option<&str> {
    if argument.len() >= 2 {
        let bytes = argument.as_bytes();
        if (bytes[0] == b'"' && bytes[argument.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[argument.len() - 1] == b'\'')
        {
            return Some(argument.trim_matches(|character| character == '"' || character == '\''));
        }
    }
    None
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test() {}
}
