use maelstrom::Runtime;
use unique_ids::{handlers, rpc};

#[tokio::main]
async fn main() {
    Runtime::new()
        .run(rpc::UniqueIdServer::new(
            handlers::UniqueIdService::default(),
        ))
        .await
        .expect("failed to run EchoServer")
}
