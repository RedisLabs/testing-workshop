use std::error::Error;

//use indoc::indoc;
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

impl<T: AsRef<str>> From<T> for RespError {
    fn from(message: T) -> Self {
        RespError::String(message.as_ref().to_string())
    }
}

// impl<E: Error> From<E> for RespError {
//     fn from(e: E) -> Self {
//         RespError::Boxed(e)
//     }
// }

fn resp_parse(data: &[u8]) -> Result<&[u8], RespError> {
    match &data {
        [b'+', data @ ..] => {
            let (line, _rest) = split_line(data);
            line.ok_or("missing end of line deliminator".into())
        }
        [b'$', data @ ..] => match split_line(data) {
            (Some(length), data) => {
                let length = std::str::from_utf8(length).map_err(|e| RespError::Boxed(e.into()))?;

                let length = length
                    .parse::<usize>()
                    .map_err(|e| RespError::Boxed(e.into()))?;

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
    assert_eq!(resp_parse(b"+hello\r\n").unwrap(), b"hello");
    assert_eq!(resp_parse(b"+hel\r\nlo\r\n").unwrap(), b"hel");
}

#[test]
fn test_resp_parse_bulk() {
    assert_eq!(
        resp_parse(b"$11\r\nhello world\r\n").unwrap(),
        b"hello world",
    );

    assert_eq!(
        resp_parse(b"$12\r\nhello\r\nworld\r\n").unwrap(),
        b"hello\r\nworld"
    );

    // TODO: Figure out how to work best with the combination of dynamic and static errors.
    // Also apply that in the Peta project (instead of the `failure` crate).
    // Maybe add a wrapped Box<dyn Error> as one of the `enum` values?
    match resp_parse(b"$") {
        Ok(data) => panic!("expected an error, got: {:?}", data),
        Err(RespError::MissingLength) => (),
        Err(e) => panic!("wrong error: {}", e),
    }

    // assert_eq!(resp_parse(b"$11").unwrap(), b"???");
    // assert_eq!(resp_parse(b"$11\r\n").unwrap(), b"???");
    // assert_eq!(resp_parse(indoc!(b"
    //     $12
    //     hello
    //     world
    //     ")), b"hello\r\nworld");
}
