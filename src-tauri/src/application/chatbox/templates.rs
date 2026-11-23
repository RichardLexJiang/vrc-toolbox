use crate::{
    adapters::file_system,
    domain::{
        chatbox::ChatboxTemplateDto,
        error::{IError, IResult},
    },
};

const TEMPLATE_LIST_FILE: &str = "template_list.json";
const TEMPLATE_MAX_SIZE: usize = 20;

pub fn write_template_list(items: Vec<ChatboxTemplateDto>) -> IResult<()> {
    if items.len() > TEMPLATE_MAX_SIZE {
        return Err(IError::Chatbox(format!(
            "template list cannot contain more than {TEMPLATE_MAX_SIZE} items"
        )));
    }
    let contents = serde_json::to_vec_pretty(&items)
        .map_err(|e| IError::Chatbox(format!("serialize template list failed,err={e:?}")))?;
    file_system::write_config_file(TEMPLATE_LIST_FILE, contents)
}

pub fn get_template_list() -> IResult<Vec<ChatboxTemplateDto>> {
    let file_contents = file_system::read_config_file(TEMPLATE_LIST_FILE)?;
    if file_contents.is_empty() {
        return Ok(vec![]);
    }
    serde_json::from_slice::<Vec<ChatboxTemplateDto>>(&file_contents)
        .map_err(|e| IError::Chatbox(format!("parse template list failed , err={e:?}")))
}
