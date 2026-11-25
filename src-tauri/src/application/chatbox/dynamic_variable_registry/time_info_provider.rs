use chrono::Local;

use crate::{
    application::chatbox::dynamic_variable_registry::provider::DynamicVariableProvider,
    domain::{chatbox::RenderContext, error::IResult},
};

pub struct LocalTimeProvider;

impl DynamicVariableProvider for LocalTimeProvider {
    fn key(&self) -> String {
        "local_time".to_string()
    }

    fn example_value(&self) -> &str {
        "20:56:59"
    }

    fn description(&self) -> String {
        format!(
            "Current local time in the format you want, example format: ${{{}(\"%Y-%m-%d %H:%M:%S\")}}",
            self.key()
        )
    }

    fn evaluate(&self, ctx: &RenderContext) -> IResult<String> {
        let format = match ctx
            .arguments()
            .get(0)
            .map(|s| s.trim_matches(|c: char| c == '\"' || c.is_whitespace()))
            .filter(|value| !value.is_empty())
        {
            None => "%H:%M:%S",
            Some(f) => f,
        };
        Ok(Local::now().format(format).to_string())
    }
}
