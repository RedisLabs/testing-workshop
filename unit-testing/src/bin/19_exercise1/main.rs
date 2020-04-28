use std::error::Error;

// use indoc::indoc;
use thiserror;
use twoway;

#[derive(thiserror::Error, Debug)]
enum RespError {
    #[error("not enough data: found {found}, expected {expected}")]
    NotEnoughData { expected: usize, found: usize },

    #[error("missing length")]
    MissingLength,

    #[error("invalid data")]
    InvalidData,

    #[error("error: {0}")]
    Boxed(Box<dyn Error>),

    #[error("error: {0}")]
    String(String),
}

impl RespError {
    fn boxed<E: Error + 'static>(e: E) -> Self {
        Self::Boxed(Box::new(e))
    }
}

impl From<&str> for RespError {
    fn from(message: &str) -> Self {
        Self::String(message.to_string())
    }
}

fn resp_parse(data: &[u8]) -> Result<&[u8], RespError> {
    match &data {
        [b'+', data @ ..] => match split_line(data) {
            (Some(line), _) => Ok(line),
            (None, _) => Err("missing end of line deliminator".into()),
        },
        [b'$', data @ ..] => match split_line(data) {
            (Some(length), data) => {
                let length = std::str::from_utf8(length).map_err(RespError::boxed)?;
                let length = length.parse::<usize>().map_err(RespError::boxed)?;

                if data.len() < length + 2 {
                    Err(RespError::NotEnoughData {
                        expected: length + 2,
                        found: data.len(),
                    }
                        .into())
                } else {
                    let data = &data[..length];
                    Ok(data)
                }
            }
            (None, _) => Err(RespError::MissingLength.into()),
        },
        _ => Err(RespError::InvalidData.into()),
    }
}

fn split_line(data: &[u8]) -> (Option<&[u8]>, &[u8]) {
    twoway::find_bytes(data, b"\r\n")
        .map(|i| {
            let line = &data[..i];
            let rest = &data[i + 2..];
            (Some(line), rest)
        })
        .unwrap_or((None, data))
}

#[test]
fn test_resp_parse_simple() {
    let table: &[(&[u8], &[u8])] = &[
        (b"+hello\r\n", b"hello"),
        (b"+hel\r\nlo\r\n", b"hel"),
    ];

    for &(input, output) in table {
        assert_eq!(resp_parse(input).unwrap(), output);
    }
}

#[test]
fn test_resp_parse_bulk() {
    let table: &[(&[u8], &[u8])] = &[
        (b"$11\r\nhello world\r\n", b"hello world"),
        (b"$12\r\nhello\r\nworld\r\n", b"hello\r\nworld"),
    ];

    for &(input, output) in table {
        assert_eq!(resp_parse(input).unwrap(), output);
    }

    match resp_parse(b"$") {
        Err(RespError::MissingLength) => (),
        Err(e) => panic!("wrong error: {}", e),
        Ok(data) => panic!("expected an error, got: {:?}", data),
    }

    match resp_parse(b"$11") {
        Err(RespError::MissingLength) => (),
        Err(e) => panic!("wrong error: {}", e),
        Ok(data) => panic!("expected an error, got: {:?}", data),
    }

    match resp_parse(b"$11\r\n") {
        Err(RespError::NotEnoughData { expected, found }) => {
            assert_eq!(expected, 11 + 2);
            assert_eq!(found, 0);
        }
        Err(e) => panic!("wrong error: {}", e),
        Ok(data) => panic!("expected an error, got: {:?}", data),
    }

    match resp_parse(b"ZZZZZZZ") {
        Err(RespError::InvalidData) => (),
        Err(e) => panic!("wrong error: {}", e),
        Ok(data) => panic!("expected an error, got: {:?}", data),
    }

    match resp_parse(b"") {
        Err(RespError::InvalidData) => (),
        Err(e) => panic!("wrong error: {}", e),
        Ok(data) => panic!("expected an error, got: {:?}", data),
    }
}
