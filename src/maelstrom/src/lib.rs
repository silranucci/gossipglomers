pub mod error;
pub mod message;
pub mod service;

use serde::Serialize;
use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::{
    error::Error,
    message::{Message, Request},
    service::Service,
};

pub struct Runtime;

impl Default for Runtime {
    fn default() -> Self {
        Self
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run<S, E, B>(&self, mut service: S) -> io::Result<()>
    where
        E: Into<Error> + Send,
        S: Service<Request<serde_json::Value>, Response = Message<B>, Error = E> + Send,
        B: Serialize + Send,
    {
        let stdin = tokio::io::stdin();
        let mut stderr = tokio::io::stderr();
        let mut lines = BufReader::new(stdin).lines();

        while let Some(line) = lines.next_line().await? {
            let request: Request<serde_json::Value> = match serde_json::from_str(&line) {
                Ok(r) => r,
                Err(e) => {
                    stderr
                        .write_all(format!("deserialize error: {e}\n").as_bytes())
                        .await
                        .ok();
                    continue;
                }
            };

            match service.call(request).await {
                Ok(message) => self.send(message).await?,
                Err(e) => {
                    let err: Error = e.into();
                    stderr.write_all(format!("{err:?}\n").as_bytes()).await.ok();
                }
            }
        }

        Ok(())
    }

    async fn send<B: Serialize>(&self, message: Message<B>) -> io::Result<()> {
        let mut stdout = tokio::io::stdout();
        let mut bytes = serde_json::to_vec(&message).expect("failed to serialize");
        bytes.push(b'\n');
        stdout.write_all(&bytes).await?;
        stdout.flush().await
    }
}
