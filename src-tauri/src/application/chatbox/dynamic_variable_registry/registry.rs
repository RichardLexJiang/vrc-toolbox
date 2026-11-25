use std::collections::HashMap;

use crate::{
    application::chatbox::dynamic_variable_registry::provider::DynamicVariableProvider,
    domain::{
        chatbox::{DynamicVariableDescription, RenderContext},
        error::{IError, IResult},
    },
};

pub struct DynamicVariableRegistry {
    providers: HashMap<String, Box<dyn DynamicVariableProvider + Send>>,
}

impl DynamicVariableRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn supported_dynamic_var_list(&self) -> Vec<DynamicVariableDescription> {
        self.providers
            .iter()
            .map(|(_, entry)| {
                DynamicVariableDescription::new(
                    entry.key(),
                    entry.example_value(),
                    entry.description(),
                )
            })
            .collect()
    }

    pub fn insert_provider(&mut self, provider: Box<dyn DynamicVariableProvider + Send>) {
        let _ = self.providers.insert(provider.key(), provider);
    }

    pub fn provide_value(&self, key: &str, ctx: RenderContext) -> IResult<String> {
        self.providers
            .get(key)
            .ok_or(IError::DynamicVariableProviderNotExist(format!(
                "provider key={key} doesn't exist"
            )))?
            .evaluate(&ctx)
    }
}
