use broadcast::{handler, rpc};
use maelstrom::Runtime;

#[tokio::main]
async fn main() {
    Runtime::new()
        .run(rpc::BroadcastServer::new(
            handler::BroadcastService::default(),
        ))
        .await
        .expect("failed to run EchoServer")
}
