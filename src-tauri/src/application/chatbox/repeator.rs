use std::{
    sync::{
        Arc, Mutex, OnceLock,
        mpsc::{self, Receiver},
    },
    thread,
    time::Duration,
};

use tauri::{AppHandle, Emitter};

use crate::{
    adapters::file_system,
    application::chatbox::message::{CHATBOX_MAX_LENGTH, send_message},
    domain::{
        chatbox::{ChatboxTemplateDto, RepeatController, RepeatStatus, RepeatTaskInfo},
        error::{IError, IResult},
    },
};

const TEMPLATE_LIST_FILE: &str = "template_list.json";
const TEMPLATE_MAX_SIZE: usize = 20;
const REPEAT_STATUS_EVENT: &str = "chatbox-repeat-status";

static REPEATING_CONTROLLER: OnceLock<Arc<Mutex<RepeatController>>> = OnceLock::new();

fn repeating_controller() -> Arc<Mutex<RepeatController>> {
    REPEATING_CONTROLLER
        .get_or_init(|| Arc::new(Mutex::new(RepeatController::default())))
        .clone()
}

pub fn start_repeating(app: AppHandle, mut messages: Vec<String>) -> IResult<RepeatStatus> {
    if messages.is_empty() {
        return Err(IError::Chatbox(
            "at least one repeat message is required".to_string(),
        ));
    }

    for message in &mut messages {
        *message = message.trim().to_string();
        if message.is_empty() || message.chars().count() > CHATBOX_MAX_LENGTH {
            return Err(IError::Chatbox(format!(
                "repeat message must contain between 1 and {CHATBOX_MAX_LENGTH} characters"
            )));
        }
    }

    let controller = repeating_controller();
    let mut state = controller
        .lock()
        .map_err(|e| IError::Chatbox(format!("repeat controller lock failed: {e:?}")))?;
    if state.active().is_some() {
        return Err(IError::Chatbox(
            "a repeating message is already active".to_string(),
        ));
    }
    let task_id = state.next_id();

    let (stop, stop_receiver) = mpsc::channel();
    let task_info = Arc::new(RepeatTaskInfo::new(task_id, messages, stop));
    let _ = state.replace_active(Some(task_info.clone()));
    drop(state);

    if let Err(error) = thread::Builder::new()
        .name("chatbox-repeat".to_string())
        .spawn({
            let task_info = task_info.clone();
            move || repeat_worker(app, task_info, stop_receiver)
        })
    {
        let _ = clear_repeat_task(task_id);
        return Err(IError::Chatbox(format!(
            "could not start repeat worker: {error}"
        )));
    } else {
        Ok(RepeatStatus::actived(task_info.messages().to_vec()))
    }
}

fn repeat_worker(app: AppHandle, task_info: Arc<RepeatTaskInfo>, stop_receiver: Receiver<()>) {
    let mut count = 0u64;
    let len = task_info.messages().len() as u64;
    loop {
        if let Err(error) = send_message(
            task_info.messages()[(count / 4 % len) as usize].clone(),
            false,
        ) {
            if clear_repeat_task(task_info.id()) {
                let _ = app.emit(
                    REPEAT_STATUS_EVENT,
                    RepeatStatus::stopped(Some(format!("{error:?}"))),
                );
            }
            return;
        }
        count += 1;
        match stop_receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => return,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
    }
}

fn clear_repeat_task(task_id: u64) -> bool {
    if let Ok(mut state) = repeating_controller().lock() {
        if state
            .active()
            .as_ref()
            .is_some_and(|task| task.id() == task_id)
        {
            state.replace_active(None);
            return true;
        }
    }
    false
}

pub fn stop_repeating() -> IResult<RepeatStatus> {
    if let Some(task) = repeating_controller()
        .lock()
        .map_err(|e| IError::Chatbox(format!("repeat controller lock failed: {e}")))?
        .replace_active(None)
    {
        let _ = task.stop().send(());
    }
    Ok(RepeatStatus::stopped(None))
}

pub fn get_repeat_status() -> IResult<RepeatStatus> {
    let controller = repeating_controller();
    let state = controller
        .lock()
        .map_err(|e| IError::Chatbox(format!("repeat controller lock failed: {e}")))?;
    Ok(match state.active() {
        Some(task) => RepeatStatus::actived(task.messages().to_vec()),
        None => RepeatStatus::stopped(None),
    })
}

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
