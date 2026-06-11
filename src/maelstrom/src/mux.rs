/// Internal multiplexer — owns stdin/stdout and demultiplexes messages between
/// the Runtime (inbound requests) and the Client (RPC responses).
///
/// Hidden from users; accessed only via `Client::new()` and `Runtime::new()`.
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock, atomic::AtomicU64},
};

use serde::Serialize;
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};

use crate::message::Message;

pub(crate) type PendingMap = Mutex<HashMap<u64, oneshot::Sender<Message<Value>>>>;

pub(crate) struct MuxInner {
    /// Source node ID — populated from the `init` message by the stdin task.
    pub(crate) src: OnceLock<String>,
    /// Monotonic counter for outbound `msg_id`s.
    pub(crate) next_msg_id: AtomicU64,
    /// Pending RPC futures: msg_id → oneshot sender.
    pub(crate) pending: Arc<PendingMap>,
    /// Channel for the Runtime to receive inbound requests.
    pub(crate) requests_tx: mpsc::UnboundedSender<Message<Value>>,
    /// Taken once by `Runtime::run`.
    pub(crate) requests_rx: Mutex<Option<mpsc::UnboundedReceiver<Message<Value>>>>,
    /// Shared stdout write channel — serialises all outbound messages.
    pub(crate) stdout_tx: mpsc::UnboundedSender<Vec<u8>>,
}

impl MuxInner {
    pub(crate) fn write<B: Serialize>(&self, msg: Message<B>) {
        let mut bytes = serde_json::to_vec(&msg).expect("failed to serialize");
        bytes.push(b'\n');
        self.stdout_tx.send(bytes).ok();
    }
}

// Process-level singleton
static MUX: OnceLock<Arc<MuxInner>> = OnceLock::new();

/// Returns the process-level Mux, initialising it on first call.
///
/// Must be called from within a Tokio runtime context because initialisation
/// spawns background tasks.
pub(crate) fn get() -> Arc<MuxInner> {
    MUX.get_or_init(|| {
        let (stdout_tx, stdout_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let (requests_tx, requests_rx) = mpsc::unbounded_channel::<Message<Value>>();

        let inner = Arc::new(MuxInner {
            src: OnceLock::new(),
            next_msg_id: AtomicU64::new(1),
            pending: Arc::new(Mutex::new(HashMap::new())),
            requests_tx,
            requests_rx: Mutex::new(Some(requests_rx)),
            stdout_tx,
        });

        tokio::spawn(stdout_writer(stdout_rx));
        tokio::spawn(stdin_reader(inner.clone()));

        inner
    })
    .clone()
}

// Background tasks

async fn stdin_reader(mux: Arc<MuxInner>) {
    use tokio::io::AsyncBufReadExt;
    let mut lines = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let msg: Message<Value> = match serde_json::from_str(&line) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("mux: deserialize error: {e}");
                continue;
            }
        };

        // Capture our node ID from the init message.
        if msg.body.kind == "init"
            && let Some(id) = msg.body.payload.get("node_id").and_then(|v| v.as_str())
        {
            mux.src.set(id.to_string()).ok();
        }

        // Route RPC responses to waiting Client::rpc futures.
        if let Some(in_reply_to) = msg.body.in_reply_to {
            let mut map = mux.pending.lock().unwrap();
            if let Some(tx) = map.remove(&in_reply_to) {
                tx.send(msg).ok();
                continue;
            }
        }

        // Everything else is a request for the Runtime.
        mux.requests_tx.send(msg).ok();
    }
}

async fn stdout_writer(mut rx: mpsc::UnboundedReceiver<Vec<u8>>) {
    use tokio::io::AsyncWriteExt;
    let mut stdout = tokio::io::stdout();
    while let Some(bytes) = rx.recv().await {
        stdout.write_all(&bytes).await.ok();
        stdout.flush().await.ok();
    }
}
