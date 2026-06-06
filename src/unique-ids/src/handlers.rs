use crate::rpc::{Generate, GenerateOk, Init, InitOk, UniqueIdApi};
use maelstrom::{
    error::ErrorCode,
    message::{Metadata, Request, Response},
};
use std::sync::{
    OnceLock,
    atomic::{AtomicU64, Ordering},
};

pub struct UniqueIdService {
    node_id: OnceLock<String>,
    counter: AtomicU64,
}

impl Default for UniqueIdService {
    fn default() -> Self {
        Self {
            node_id: OnceLock::new(),
            counter: AtomicU64::new(0),
        }
    }
}

impl UniqueIdApi for UniqueIdService {
    async fn init(&self, req: Request<Init>) -> Result<Response<InitOk>, ErrorCode> {
        let (metadata, body) = req.into_parts();
        self.node_id.set(body.node_id).ok();

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

    async fn generate(&self, req: Request<Generate>) -> Result<Response<GenerateOk>, ErrorCode> {
        let metadata = req.metadata();

        Ok(Response::new(
            Metadata {
                src: metadata.dest,
                dest: metadata.src,
                kind: "generate_ok".to_string(),
                msg_id: None,
                in_reply_to: metadata.msg_id,
            },
            GenerateOk {
                id: simple_unique_id(&self.node_id, &self.counter),
            },
        ))
    }
}

fn simple_unique_id(node_id: &OnceLock<String>, counter: &AtomicU64) -> String {
    let node_id = node_id.get().map(|s| s.as_str()).unwrap();
    let seq = counter.fetch_add(1, Ordering::Relaxed);
    format!("{}-{}", node_id, seq)
}
