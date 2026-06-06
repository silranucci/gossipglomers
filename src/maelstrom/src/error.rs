use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::message::MessageId;

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

impl From<ErrorCode> for Error {
    fn from(code: ErrorCode) -> Self {
        Error::new(code, String::new())
    }
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
    /// Whether the error is definite or not
    pub fn is_definite(&self) -> bool {
        matches!(self, ErrorCode::Timeout | ErrorCode::Crash)
    }
}
