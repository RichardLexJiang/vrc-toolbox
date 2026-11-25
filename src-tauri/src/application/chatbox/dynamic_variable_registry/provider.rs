use crate::domain::{chatbox::RenderContext, error::IResult};

pub trait DynamicVariableProvider {
    fn key(&self) -> String;

    fn example_value(&self) -> &str;

    fn description(&self) -> String;

    fn evaluate(&self, ctx: &RenderContext) -> IResult<String>;
}
