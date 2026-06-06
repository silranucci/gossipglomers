use crate::rpc::{Echo, EchoApi, EchoOk, Init, InitOk};
use maelstrom::{
    error::ErrorCode,
    message::{Metadata, Request, Response},
};
use std::sync::OnceLock;

pub struct EchoService {
    node_id: OnceLock<String>,
    node_ids: OnceLock<Vec<String>>,
}

impl Default for EchoService {
    fn default() -> Self {
        Self {
            node_id: OnceLock::new(),
            node_ids: OnceLock::new(),
        }
    }
}

impl EchoApi for EchoService {
    async fn init(&self, req: Request<Init>) -> Result<Response<InitOk>, ErrorCode> {
        let (metadata, body) = req.into_parts();
        self.node_id.set(body.node_id).ok();
        self.node_ids.set(body.node_ids).ok();
        Ok(Response::new(
            Metadata {
                src: metadata.dest,
                dest: metadata.src,
                kind: "init_ok".to_string(),
                msg_id: None,
                in_reply_to: metadata.msg_id,
            },
            InitOk {},
        ))
    }

    async fn echo(&self, req: Request<Echo>) -> Result<Response<EchoOk>, ErrorCode> {
        let (metadata, body) = req.into_parts();
        Ok(Response::new(
            Metadata {
                src: metadata.dest,
                dest: metadata.src,
                kind: "echo_ok".to_string(),
                msg_id: None,
                in_reply_to: metadata.msg_id,
            },
            EchoOk { echo: body.echo },
        ))
    }
}
