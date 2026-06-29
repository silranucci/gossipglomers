use std::sync::{Mutex, OnceLock};

use crate::rpc::BroadcastApi;
use crate::rpc::{Broadcast, BroadcastOk, Init, InitOk, Read, ReadOk, Topology, TopologyOk};
use maelstrom::Client;
use maelstrom::{
    error::ErrorCode,
    message::{Request, Response},
};

pub struct BroadcastService {
    node_id: OnceLock<String>,
    node_ids: OnceLock<Vec<String>>,
    neighbors: OnceLock<Vec<String>>,
    messages: Mutex<Vec<i64>>,
    client: Client,
}

impl Default for BroadcastService {
    fn default() -> Self {
        Self {
            node_id: OnceLock::new(),
            node_ids: OnceLock::new(),
            neighbors: OnceLock::new(),
            messages: Mutex::new(vec![]),
            client: Client::new(),
        }
    }
}

impl BroadcastApi for BroadcastService {
    async fn init(&self, req: Request<Init>) -> Result<Response<InitOk>, ErrorCode> {
        let body = req.body();
        self.node_id.set(body.node_id.clone()).ok();
        self.node_ids.set(body.node_ids.clone()).ok();
        Ok(Response::new(InitOk {}))
    }

    async fn topology(&self, req: Request<Topology>) -> Result<Response<TopologyOk>, ErrorCode> {
        let (_, body) = req.into_parts();
        if let Some((_, neighbors)) = body
            .topology
            .into_iter()
            .find(|(key, _)| self.node_id.get().is_some_and(|id| id == key))
        {
            self.neighbors.set(neighbors).ok();
        }
        Ok(Response::new(TopologyOk {}))
    }

    async fn broadcast(&self, req: Request<Broadcast>) -> Result<Response<BroadcastOk>, ErrorCode> {
        let message = req.body().message;
        self.messages
            .lock()
            .map_err(|_| ErrorCode::Crash)?
            .push(message);
        if let Some(neighbors) = self.neighbors.get() {
            for neighbor in neighbors {
                self.client.rpc(neighbor, "broadcast", &message).await.ok();
            }
        }
        Ok(Response::new(BroadcastOk {}))
    }

    async fn read(&self, _req: Request<Read>) -> Result<Response<ReadOk>, ErrorCode> {
        if let Ok(res) = self.messages.lock() {
            Ok(Response::new(ReadOk {
                messages: res.clone(),
            }))
        } else {
            Err(ErrorCode::Crash)
        }
    }
}
