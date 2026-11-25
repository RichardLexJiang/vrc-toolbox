use vrc_osc_api::vrchat::chatbox;

use crate::{
    adapters::{self, file_system},
    application::chatbox::dynamic_variable_registry,
    domain::{
        chatbox::{ChatboxHistoryDto, RenderContext},
        error::{IError, IResult},
    },
};

const CHATBOX_HISTORY_FILE: &str = "chatbox_history.json";
const HISTORY_MAX_SIZE: usize = 200;

pub fn send_message(message: String, append_his: bool) -> IResult<()> {
    if append_his {
        let _ = append_history(ChatboxHistoryDto::new(message.clone()));
    }
    send_raw_message(render_message(&message, RenderContext::default())?)
}

pub fn render_message(message: &str, ctx: RenderContext) -> IResult<String> {
    ParsedMessage::new(message)?.render(ctx)
}

pub fn send_raw_message(message: String) -> IResult<()> {
    let message = chatbox::ChatboxInput::new(&message);
    let client = adapters::vrc_osc_api::get_client()?;
    client
        .write()
        .map_err(|e| IError::VrcOscClientLock(format!("lock error={e:?}")))?
        .send(message)
        .map(|_| ())
        .map_err(|e| IError::VrcOscApi(format!("Send message failed,err={e:?}")))
}

pub fn read_history() -> IResult<Vec<ChatboxHistoryDto>> {
    let file_contents = file_system::read_config_file(CHATBOX_HISTORY_FILE)?;
    if file_contents.is_empty() {
        return Ok(vec![]);
    }
    serde_json::from_slice::<Vec<ChatboxHistoryDto>>(&file_contents)
        .map_err(|e| IError::Chatbox(format!("parse history failed , err={e:?}")))
}

fn append_history(item: ChatboxHistoryDto) -> IResult<()> {
    let mut list = read_history()?;
    list.push(item);
    while list.len() > HISTORY_MAX_SIZE {
        let _ = list.remove(0);
    }
    write_history(list)
}

fn write_history(item: Vec<ChatboxHistoryDto>) -> IResult<()> {
    let contents = serde_json::to_vec_pretty(&item)
        .map_err(|e| IError::Chatbox(format!("serialize history failed,err={e:?}")))?;
    file_system::write_config_file(CHATBOX_HISTORY_FILE, contents)
}

#[derive(Debug, Clone)]
pub struct ParsedMessage {
    parts: Vec<MessagePart>,
}

impl ParsedMessage {
    pub fn new(content: &str) -> IResult<Self> {
        let mut parts = Vec::new();
        let mut raw_start = 0;
        let mut search_start = 0;

        while let Some(relative_start) = content[search_start..].find("${") {
            let token_start = search_start + relative_start;
            let Some(relative_end) = content[token_start + 2..].find('}') else {
                break;
            };
            let token_end = token_start + 2 + relative_end;
            let source = &content[token_start..=token_end];
            let expression = &content[token_start + 2..token_end];

            push_raw(&mut parts, &content[raw_start..token_start]);

            let (key, arguments) = match expression.find('(') {
                Some(argument_start) if expression.ends_with(')') => (
                    &expression[..argument_start],
                    Some(&expression[argument_start + 1..expression.len() - 1]),
                ),
                _ => (expression, None),
            };

            parts.push(MessagePart::DynamicVariable {
                source: source.to_string(),
                key: key.to_string(),
                arguments: arguments
                    .map_or_default(|args| args.split(',').map(str::to_string).collect()),
            });
            raw_start = token_end + 1;
            search_start = raw_start;
        }

        push_raw(&mut parts, &content[raw_start..]);
        Ok(Self { parts })
    }

    pub fn render(&self, mut ctx: RenderContext) -> IResult<String> {
        let mut output = String::new();
        for part in &self.parts {
            match part {
                MessagePart::RawText(text) => output.push_str(text),
                MessagePart::DynamicVariable {
                    source,
                    key,
                    arguments,
                } => {
                    for ele in arguments {
                        (&mut ctx).arguments_mut().push(ele.to_string());
                    }
                    match dynamic_variable_registry::provide_value(key, ctx.clone()) {
                        Ok(result) => output.push_str(&result),
                        Err(_) => {
                            // todo log
                            output.push_str(source);
                        }
                    };
                }
            }
        }
        Ok(output)
    }
}

#[derive(Debug, Clone)]
pub enum MessagePart {
    RawText(String),
    DynamicVariable {
        source: String,
        key: String,
        arguments: Vec<String>,
    },
}

fn push_raw(parts: &mut Vec<MessagePart>, text: &str) {
    if text.is_empty() {
        return;
    }
    if let Some(MessagePart::RawText(existing)) = parts.last_mut() {
        existing.push_str(text);
    } else {
        parts.push(MessagePart::RawText(text.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::{MessagePart, ParsedMessage};
    use crate::application::chatbox::dynamic_variable::DynamicVariable;

    #[test]
    fn parses_raw_text_and_dynamic_variables() {
        let parsed =
            ParsedMessage::new("CPU: ${cpu_usage}, dots ${dots(6)}").expect("message should parse");

        assert!(matches!(
            &parsed.parts()[0],
            MessagePart::RawText(text) if text == "CPU: "
        ));

        assert!(matches!(
            &parsed.parts()[2],
            MessagePart::RawText(text) if text == ", dots "
        ));
    }

    #[test]
    fn preserves_unknown_variables_as_raw_text() {
        let parsed = ParsedMessage::new("hello ${unknown}").expect("message should parse");

        assert!(matches!(
            parsed.parts(),
            [MessagePart::RawText(text)] if text == "hello ${unknown}"
        ));
    }

    #[test]
    fn rejects_invalid_dot_count() {
        assert!(ParsedMessage::new("${dots(7)}").is_err());
        assert!(ParsedMessage::new("${dots(nope)}").is_err());
    }
}
