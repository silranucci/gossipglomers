pub mod message;

use std::io::{self, BufRead, BufReader, Stderr, Stdin, Stdout, Write};

use serde::Serialize;

pub struct Runtime {
    stdin: Stdin,
    stdout: Stdout,
    stderr: Stderr,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            stdin: io::stdin(),
            stdout: io::stdout(),
            stderr: io::stderr(),
        }
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run<T, E, F>(&self, f: F) -> io::Result<()>
    where
        T: Serialize,
        E: Into<message::Error> + Serialize,
        F: Fn(message::Message) -> Result<T, E>,
    {
        let mut reader = BufReader::new(&self.stdin).lines();

        #[allow(clippy::while_let_on_iterator)]
        while let Some(maybe_line) = reader.next() {
            match maybe_line {
                Ok(line) => {
                    // 1. Parse JSON
                    let request: message::Message = serde_json::from_str(line.as_str())
                        .expect("Something went wrong when pippoing");
                    // 2. Look up handler in a HashMap/Router

                    // 3. Execute handler
                    // 4. Write result to stdout
                    match f(request.clone()) {
                        Ok(body) => {
                            self.write_response(request, body)?;
                        }
                        // TODO: to send to stderr?
                        Err(e) => {
                            let message_error: message::Error = e.into();
                            serde_json::to_writer(&self.stdout, &message_error)
                                .expect("failed to serialize");
                        }
                    }
                }
                Err(_) => {
                    let mut handle = self.stderr.lock();
                    handle
                        .write_all(b"An error occured while reading stdin")
                        .expect("failed to read to stderr");
                }
            }
        }
        // apply handler
        Ok(())
    }

    fn write_response<T: Serialize>(&self, request: message::Message, body: T) -> io::Result<()> {
        let response = message::Message {
            src: request.dest,
            dest: request.src,
            body: serde_json::to_value(body).expect("failed to serialize body"),
        };
        self.send(response)
    }

    fn send(&self, message: message::Message) -> io::Result<()> {
        let mut handle = self.stdout.lock();
        serde_json::to_writer(&mut handle, &message).expect("failed to serialize");
        handle.write_all(b"\n")?;
        handle.flush()
    }
}
