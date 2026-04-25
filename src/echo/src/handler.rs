use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Echo {
    echo: String,
}

#[derive(Debug, Serialize)]
pub enum Error {
    InvalidEchoRequest,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidEchoRequest => write!(f, "Invalid echo request"),
        }
    }
}

impl From<Error> for maelstrom::message::Error {
    fn from(err: Error) -> Self {
        maelstrom::message::Error::new(
            maelstrom::message::ErrorCode::PreconditionFailed,
            format!("{}", err),
        )
    }
}

pub fn handler(msg: serde_json::Value) -> Result<Echo, Error> {
    let echo_msg: Echo = serde_json::from_value(msg).map_err(|_| Error::InvalidEchoRequest)?;
    Ok(Echo {
        echo: echo_msg.echo,
    })
}
