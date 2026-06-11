pub mod client;
pub mod error;
pub mod message;
mod mux;
pub mod service;

pub use client::Client;

use serde::Serialize;
use std::io;

use crate::{
    error::Error,
    message::{Message, Request},
    service::Service,
};

pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Self
    }

    pub async fn run<S, E, B>(&self, mut service: S) -> io::Result<()>
    where
        E: Into<Error> + Send,
        S: Service<Request<serde_json::Value>, Response = Message<B>, Error = E> + Send,
        B: Serialize + Send,
    {
        let mux = mux::get();
        let mut requests_rx = mux
            .requests_rx
            .lock()
            .unwrap()
            .take()
            .expect("Runtime::run called more than once");

        while let Some(msg) = requests_rx.recv().await {
            let request = Request::from_message(msg);
            match service.call(request).await {
                Ok(reply) => mux.write(reply),
                Err(e) => {
                    let err: Error = e.into();
                    eprintln!("{err:?}");
                }
            }
        }

        Ok(())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
