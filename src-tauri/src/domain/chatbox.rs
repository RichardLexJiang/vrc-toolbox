use std::{
    sync::{Arc, mpsc::Sender},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// chat box history record
pub struct ChatboxHistoryDto {
    origin_content: String,
    parsed_content: Option<String>,
    send_time: u64,
}

impl ChatboxHistoryDto {
    pub fn new(origin_content: String) -> Self {
        Self {
            origin_content,
            parsed_content: None,
            send_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatboxTemplateDto {
    #[serde(default)]
    name: String,
    content: String,
    // display_duration_seconds: usize,
}

#[derive(Default)]
pub struct RepeatController {
    id: u64,
    active: Option<Arc<RepeatTaskInfo>>,
}

impl RepeatController {
    pub fn next_id(&mut self) -> u64 {
        self.id = self.id + 1;
        self.id
    }

    pub fn replace_active(
        &mut self,
        task: Option<Arc<RepeatTaskInfo>>,
    ) -> Option<Arc<RepeatTaskInfo>> {
        if let Some(task) = task {
            self.active.replace(task)
        } else {
            self.active.take()
        }
    }

    pub fn active(&self) -> Option<Arc<RepeatTaskInfo>> {
        self.active.clone()
    }
}

pub struct RepeatTaskInfo {
    id: u64,
    messages: Vec<String>,
    stop: Sender<()>,
}

impl RepeatTaskInfo {
    pub fn new(id: u64, messages: Vec<String>, stop: Sender<()>) -> Self {
        Self { id, messages, stop }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn stop(&self) -> &Sender<()> {
        &self.stop
    }

    pub fn messages(&self) -> &[String] {
        &self.messages
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepeatStatus {
    pub active: bool,
    pub messages: Option<Vec<String>>,
    pub error: Option<String>,
}

impl RepeatStatus {
    pub fn actived(messages: Vec<String>) -> Self {
        Self {
            active: true,
            messages: Some(messages),
            error: None,
        }
    }

    pub fn stopped(error: Option<String>) -> Self {
        Self {
            active: false,
            messages: None,
            error,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicVariableDescription {
    pub token: String,
    pub example_value: String,
    pub description: String,
}

impl DynamicVariableDescription {
    pub fn new(
        token: impl Into<String>,
        example_value: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            token: token.into(),
            example_value: example_value.into(),
            description: description.into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RenderContext {
    totle_tick: usize,
    template_tick: usize,
    is_repeating: bool,
    arguments: Vec<String>,
}

impl RenderContext {
    pub fn totle_tick(&self) -> usize {
        self.totle_tick
    }

    pub fn template_tick(&self) -> usize {
        self.template_tick
    }

    pub fn totle_tick_mut(&mut self) -> &mut usize {
        &mut self.totle_tick
    }

    pub fn template_tick_mut(&mut self) -> &mut usize {
        &mut self.template_tick
    }

    pub fn is_repeating_mut(&mut self) -> &mut bool {
        &mut self.is_repeating
    }

    pub fn is_repeating(&self) -> bool {
        self.is_repeating
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn arguments_mut(&mut self) -> &mut Vec<String> {
        &mut self.arguments
    }
}

#[cfg(test)]
mod tests {
    use super::ChatboxTemplateDto;

    #[test]
    fn loads_legacy_templates_without_names() {
        let template = serde_json::from_str::<ChatboxTemplateDto>(r#"{"content":"Back soon"}"#)
            .expect("legacy template should deserialize");

        assert_eq!(template.name, "");
        assert_eq!(template.content, "Back soon");
    }
}
