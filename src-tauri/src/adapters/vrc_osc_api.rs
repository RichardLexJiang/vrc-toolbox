use std::{
    net::SocketAddr,
    sync::{Arc, OnceLock, RwLock},
};

use vrc_osc_api::client::VRCOSCClient;

use crate::domain::error::{IError, IResult};

pub type ARClient = Arc<RwLock<VRCOSCClient>>;

static CLIENT: OnceLock<ARClient> = OnceLock::new();

pub fn init_client(target: &str) -> IResult<ARClient> {
    let target = target
        .parse::<SocketAddr>()
        .map_err(|e| IError::VrcOscApi(format!("ip addr incorrect. err:{e}")))?;
    let client = CLIENT
        .get_or_init(|| {
            Arc::new(RwLock::new(
                VRCOSCClient::new()
                    .map(|mut c| {
                        c.replace_target(target);
                        c
                    })
                    .unwrap(),
            ))
        })
        .clone();
    client
        .write()
        .map_err(|e| IError::VrcOscClientLock(e.to_string()))?
        .start()
        .map_err(|e| IError::VrcOscClientInit(format!("start client failed,err={e}")))?;
    Ok(client)
}

pub fn get_client() -> IResult<ARClient> {
    CLIENT
        .get()
        .map(Clone::clone)
        .ok_or_else(|| IError::VrcOscApi("client not initialized".to_string()))
}

pub fn reset_target(target: &str) -> IResult<()> {
    let target = target
        .parse::<SocketAddr>()
        .map_err(|e| IError::VrcOscApi(format!("ip addr incorrect. err:{e}")))?;
    get_client().and_then(|c| {
        c.write()
            .map_err(|e| IError::VrcOscClientLock(e.to_string()))
            .map(|mut c| {
                c.replace_target(target);
                //todo restart
            })
    })
}
