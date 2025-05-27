use core::panic;
use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    str,
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000")?;

    for stream in listener.incoming() {
        let mut conn = Connection { stream: stream? };

        let path = conn.handle_stream()?;
        dbg!(&path);
        conn.reply(path)?;
    }

    Ok(())
}

struct Connection {
    stream: TcpStream,
}

impl Connection {
    fn handle_stream(&mut self) -> Result<String> {
        let mut buf = [0; 1024];
        let stream_len = self.stream.read(&mut buf)?;
        if stream_len == 0 {
            panic!();
        }

        let text = str::from_utf8(&buf[0..stream_len]).unwrap();
        let mut req = text.lines().next().unwrap().split_whitespace();
        let Some("GET") = req.next() else {
            panic!();
        };
        let Some(path) = req.next() else {
            panic!();
        };
        let Some("HTTP/1.1") = req.next() else {
            panic!();
        };

        Ok(path.to_string())
    }

    fn reply(&mut self, path: String) -> Result<()> {
        dbg!("reply");
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n Hello!\r\n";
        self.stream.write(response.as_bytes())?;

        Ok(())
    }
}
