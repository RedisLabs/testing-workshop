use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn resp_parse(reader: impl Read) -> Result<String> {
    let mut reader = BufReader::new(reader);

    let mut resp_type = [0; 1];
    reader.read_exact(&mut resp_type)?;

    match resp_type[0] {
        b'+' => parse_simple_string(&mut reader),
        _ => Err(format!("Illegal RESP: {}", resp_type[0] as char).into()),
    }
}

fn parse_simple_string(mut reader: impl BufRead) -> Result<String> {
    let mut data = String::new();
    reader.read_line(&mut data)?;
    let data = data.trim_end().to_string();
    Ok(data)
}

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    stream.write_all(b"PING\r\n")?;

    let reply = resp_parse(stream)?;
    print!("{:?}", reply);

    Ok(())
}
