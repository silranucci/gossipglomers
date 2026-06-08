use crate::rpc::{Echo, EchoApi, EchoOk, Init, InitOk};
use maelstrom::{
    error::ErrorCode,
    message::{Request, Response},
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
        let body = req.body();
        self.node_id.set(body.node_id.clone()).ok();
        self.node_ids.set(body.node_ids.clone()).ok();
        Ok(Response::new(InitOk {}))
    }

    async fn echo(&self, req: Request<Echo>) -> Result<Response<EchoOk>, ErrorCode> {
        Ok(Response::new(EchoOk {
            echo: req.body().echo.clone(),
        }))
    }
}
