use serde::{Deserialize, Serialize};

/// Top-level Maelstrom wire message.
///
/// Serialises as:
/// ```json
/// {"src":"n1","dest":"c1","body":{"type":"echo_ok","in_reply_to":1,"echo":"hi"}}
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message<B> {
    pub src: String,
    pub dest: String,
    pub body: Body<B>,
}

/// The nested `body` object of a Maelstrom message.
///
/// `kind` serialises as `"type"`. Any extra payload fields are flattened
/// alongside `type`, `msg_id`, and `in_reply_to`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Body<B> {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<u64>,
    #[serde(flatten)]
    pub payload: B,
}

/// Convenience struct for constructing and reading message metadata.
///
/// Combines the top-level routing fields (`src`, `dest`) with the body-level
/// envelope fields (`kind`, `msg_id`, `in_reply_to`) so handlers don't need
/// to reach into the nested wire types.
#[derive(Clone, Debug)]
pub struct Metadata {
    pub src: String,
    pub dest: String,
    pub kind: String,
    pub msg_id: Option<u64>,
    pub in_reply_to: Option<u64>,
}

#[derive(Deserialize, Clone)]
pub struct Request<B>(Message<B>);

impl<B> Request<B> {
    pub fn new(metadata: Metadata, payload: B) -> Self {
        Self(Message {
            src: metadata.src,
            dest: metadata.dest,
            body: Body {
                kind: metadata.kind,
                msg_id: metadata.msg_id,
                in_reply_to: metadata.in_reply_to,
                payload,
            },
        })
    }

    /// Returns a snapshot of the envelope fields.
    pub fn metadata(&self) -> Metadata {
        Metadata {
            src: self.0.src.clone(),
            dest: self.0.dest.clone(),
            kind: self.0.body.kind.clone(),
            msg_id: self.0.body.msg_id,
            in_reply_to: self.0.body.in_reply_to,
        }
    }

    pub fn body(&self) -> &B {
        &self.0.body.payload
    }

    pub(crate) fn from_message(msg: Message<B>) -> Self {
        Self(msg)
    }

    pub fn into_parts(self) -> (Metadata, B) {
        (
            Metadata {
                src: self.0.src,
                dest: self.0.dest,
                kind: self.0.body.kind,
                msg_id: self.0.body.msg_id,
                in_reply_to: self.0.body.in_reply_to,
            },
            self.0.body.payload,
        )
    }
}

/// Handler-facing response. Carries the typed payload and an optional metadata
/// override. When `metadata` is `None` the runtime derives routing automatically
/// from the incoming request (swap `src`/`dest`, set `in_reply_to`).
pub struct Response<B> {
    pub(crate) metadata: Option<Metadata>,
    pub(crate) payload: B,
}

impl<B> Response<B> {
    /// Create a response with no metadata override; the framework fills in
    /// `src`, `dest`, `kind`, and `in_reply_to` automatically.
    pub fn new(payload: B) -> Self {
        Self {
            metadata: None,
            payload,
        }
    }

    /// Create a response with explicit routing metadata.
    pub fn with_metadata(metadata: Metadata, payload: B) -> Self {
        Self {
            metadata: Some(metadata),
            payload,
        }
    }

    pub fn body(&self) -> &B {
        &self.payload
    }

    pub fn into_parts(self) -> (Option<Metadata>, B) {
        (self.metadata, self.payload)
    }
}

pub type MessageId = u64;
