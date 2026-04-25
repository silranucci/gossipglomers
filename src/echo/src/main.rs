mod handler;

use maelstrom::Runtime;

fn main() {
    let runtime = Runtime::new();
    runtime
        .run(|msg| handler::handler(msg.body))
        .expect("failed");
}
