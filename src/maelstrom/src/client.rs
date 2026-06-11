use std::sync::{Arc, atomic::Ordering};

use serde::Serialize;
use serde_json::Value;
use tokio::sync::oneshot;

use crate::{
    error::{Error, ErrorCode},
    message::{Body, Message},
    mux::{self, MuxInner},
};

/// A cloneable handle for sending outbound Maelstrom messages.
///
/// Works like `reqwest::Client` — construct it independently, clone freely,
/// store in service structs. The underlying I/O is shared with the Runtime
/// via a hidden process-level multiplexer.
///
/// # Example
/// ```rust
/// let client = maelstrom::Client::new();
///
/// // fire-and-forget
/// client.send("n2", "broadcast", &BroadcastBody { message: 42 });
///
/// // await a response
/// let reply = client.rpc("n2", "read", &ReadBody {}).await?;
/// ```
#[derive(Clone)]
pub struct Client {
    mux: Arc<MuxInner>,
}

impl Client {
    /// Create a new client. May be called anywhere within a Tokio runtime.
    pub fn new() -> Self {
        Self { mux: mux::get() }
    }

    /// Fire-and-forget. No `msg_id` is attached; no response is expected.
    pub fn send<B: Serialize>(&self, dest: impl Into<String>, kind: impl Into<String>, body: &B) {
        self.mux.write(Message {
            src: self.src(),
            dest: dest.into(),
            body: Body {
                kind: kind.into(),
                msg_id: None,
                in_reply_to: None,
                payload: serde_json::to_value(body).expect("failed to serialize"),
            },
        });
    }

    /// Send a message and await the response.
    pub async fn rpc<B: Serialize>(
        &self,
        dest: impl Into<String>,
        kind: impl Into<String>,
        body: &B,
    ) -> Result<Message<Value>, Error> {
        let msg_id = self.mux.next_msg_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.mux.pending.lock().unwrap().insert(msg_id, tx);

        self.mux.write(Message {
            src: self.src(),
            dest: dest.into(),
            body: Body {
                kind: kind.into(),
                msg_id: Some(msg_id),
                in_reply_to: None,
                payload: serde_json::to_value(body).expect("failed to serialize"),
            },
        });

        rx.await.map_err(|_| Error::from(ErrorCode::Crash))
    }

    fn src(&self) -> String {
        self.mux.src.get().cloned().unwrap_or_default()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
