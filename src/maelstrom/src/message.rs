use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeId(u64);

impl Node {
    pub fn new(n: impl Into<String>) -> Self {
        Self(n.into())
    }
}

type MessageId = u64;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum InitBody {
    Init {
        msg_id: MessageId,
        node_id: NodeId,
        node_ids: Vec<NodeId>,
    },
    InitOk {
        in_reply_to: MessageId,
    },
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub enum ErrorCode {
    /// Indicates that the requested operation could not be completed within a timeout.
    Timeout = 0,
    /// Thrown when a client sends an RPC request to a node which does not exist.
    NodeNotFound = 1,
    /// Use this error to indicate that a requested operation is not supported.
    NotSupported = 10,
    /// Indicates that the operation definitely cannot be performed at this time.
    TemporarilyUnavailable = 11,
    /// The client's request did not conform to the server's expectations.
    MalformedRequest = 12,
    /// Indicates that some kind of general, indefinite error occurred.
    Crash = 13,
    /// Indicates that some kind of general, definite error occurred.
    Abort = 14,
    /// The client requested an operation on a key which does not exist.
    KeyDoesNotExist = 20,
    /// The client requested the creation of a key which already exists.
    KeyAlreadyExists = 21,
    /// The requested operation expected some conditions to hold, and those conditions were not met.
    PreconditionFailed = 22,
    /// The requested transaction has been aborted because of a conflict.
    TxnConflict = 30,
}

impl ErrorCode {
    pub fn is_definite(&self) -> bool {
        match self {
            ErrorCode::Timeout | ErrorCode::Crash => false,
            _ => true,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    in_reply_to: MessageId,
    code: ErrorCode,
    text: String,
}

impl Error {
    pub fn new(code: ErrorCode, message: String) -> Self {
        Self {
            in_reply_to: 1,
            code,
            text: message,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    // A string identifying the node this message came from
    pub src: Node,
    // A string identifying the node this message is to
    pub dest: Node,
    pub body: serde_json::Value,
}
