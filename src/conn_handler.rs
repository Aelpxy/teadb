use crate::logger;
use may::net::TcpStream;
use std::io::{Read, Result, Write};

pub fn handle_connection(mut stream: TcpStream) -> Result<()> {
  loop {
    let mut buf: Vec<u8> = vec![0; 1024];

    let bytes_read = match stream.read(&mut buf) {
      Ok(0) => {
        logger::info("Connection closed");
        return Ok(());
      }
      Ok(n) => n,
      Err(e) => {
        logger::warn("Connection error");
        logger::trace(&e.to_string(), "conn_handler::handle_connection");
        return Err(e);
      }
    };

    match stream.write_all(&buf[..bytes_read]) {
      Ok(_) => {
        continue;
      }
      Err(e) => {
        logger::warn("Connection error");
        logger::trace(&e.to_string(), "conn_handler::handle_connection");
        return Err(e);
      }
    }
  }
}
