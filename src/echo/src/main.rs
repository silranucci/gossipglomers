mod handler;
mod rpc;

use maelstrom::Runtime;

#[tokio::main]
async fn main() {
    Runtime::new()
        .run(rpc::EchoServer::<handler::EchoService>::default())
        .await
        .expect("failed to run EchoServer")
}
