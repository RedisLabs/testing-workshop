use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    stream.write_all(b"PING\r\n")?;

    let mut reader = BufReader::new(stream);

    let mut data = String::new();
    reader.read_line(&mut data)?;

    println!("{:?}", data);

    Ok(())
}
