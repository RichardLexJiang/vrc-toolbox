
use vrc_osc_api::vrchat::chatbox;

use crate::{
    adapters::{self, file_system},
    domain::{
        chatbox::{
            ChatboxHistoryDto, 
        },
        error::{IError, IResult},
    },
};

const CHATBOX_HISTORY_FILE: &str = "chatbox_history.json";
const HISTORY_MAX_SIZE: usize = 200;
pub const CHATBOX_MAX_LENGTH: usize = 144;

pub fn send_message(message: String, append_his: bool) -> IResult<()> {
    if append_his {
        let _ = append_history(ChatboxHistoryDto::new(message.clone()));
    }
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
