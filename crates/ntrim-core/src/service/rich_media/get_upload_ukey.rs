use std::sync::Arc;
use anyhow::{anyhow, Error};
use crate::await_response;
use crate::bot::Bot;

type UKey = (Vec<u8>, Vec<(u32, u32)>);

impl Bot {
    pub async fn get_upload_key(self: &Arc<Bot>) -> Result<UKey, Error> {
        let value = await_response!(tokio::time::Duration::from_secs(5), async {
        let rx = Bot::_request_upload_ukey(self).await;
        if let Some(rx) = rx {
            rx.await.map_err(|e| Error::new(e))
        } else {
            Err(Error::msg("Unable to get upload key: tcp connection exception"))
        }
    }, |value| {
        Ok(value)
    }, |e| {
        Err(e)
    })?.ok_or(anyhow!("UploadUKeyRsp is none"))?;
        return Ok(value);
    }
}