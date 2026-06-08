use crate::rpc::{Generate, GenerateOk, Init, InitOk, UniqueIdApi};
use maelstrom::{error::ErrorCode, message::{Request, Response}};
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
        self.node_id.set(req.body().node_id.clone()).ok();
        Ok(Response::new(InitOk {}))
    }

    async fn generate(&self, _req: Request<Generate>) -> Result<Response<GenerateOk>, ErrorCode> {
        let node_id = self.node_id.get().map(|s| s.as_str()).unwrap_or("unknown");
        let seq = self.counter.fetch_add(1, Ordering::Relaxed);
        Ok(Response::new(GenerateOk { id: format!("{}-{}", node_id, seq) }))
    }
}
