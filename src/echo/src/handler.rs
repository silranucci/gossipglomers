use crate::rpc::{Echo, EchoApi, EchoOk, Init, InitOk};
use maelstrom::{
    error::ErrorCode,
    message::{Metadata, Request, Response},
};

pub struct EchoService;

impl EchoApi for EchoService {
    async fn init(req: Request<Init>) -> Result<Response<InitOk>, ErrorCode> {
        let metadata = req.metadata();
        Ok(Response::new(
            Metadata {
                src: metadata.dest,
                dest: metadata.src,
                kind: "init_ok".to_string(),
                msg_id: None,
                in_reply_to: metadata.msg_id,
            },
            InitOk {},
        ))
    }

    async fn echo(req: Request<Echo>) -> Result<Response<EchoOk>, ErrorCode> {
        let (metadata, body) = req.into_parts();
        Ok(Response::new(
            Metadata {
                src: metadata.dest,
                dest: metadata.src,
                kind: "echo_ok".to_string(),
                msg_id: None,
                in_reply_to: metadata.msg_id,
            },
            EchoOk { echo: body.echo },
        ))
    }
}
