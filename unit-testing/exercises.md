# Exercises

## Unit tests

Implement a parser for a subset of [the RESP protocol](https://redis.io/topics/protocol).

In the first stage, we will parse _Simple Strings_ and _Bulk Strings_ (see description on the RESP protocol page).

We use the notation `b"..."` (valid both in Python and Rust) to emphasize that these are _binary_ strings, not Unicode strings.
Redis doesn't know or care what data you put into the strings, so while they can be valid UTF-8 they can also be any plain binary data.

Therefore, it's important not to make any assumptions not made explicit by the protocol.

### Examples

_Simple String_:

    +OK

    Raw: b"+OK\r\n"

_Bulk String_:

    $6
    foobar

    Raw: b"$6\r\nfoobar\r\n"


### Steps

1. Write a function named `resp_parse` that takes a sequence of bytes as an argument and extracts a Simple String
   or a Bulk String according to the first bytes. The function should returns either a sequence of bytes or an error value
   (`Result` in Rust or a custom exception in Python).
   
   Python (syntax is Python 3):
   
   ```
   def resp_parse(data: bytes) -> bytes:
       if ...:
           raise RespError(...)
       pass
   ```

   Rust:
   
   ```
   fn resp_parse(data: &[u8]) -> Result<&[u8], RespError> {
   }
   ```
    
2. Write a test suite (using [pytest](https://pytest.org/) for Python or the built-in Rust test functionality) that 
   verifies correct parsing of the above examples. Be sure to handle:
   
   - Empty Bulk Strings (`b"$0\r\n\r\n"`)
   - Null Bulk Strings (`b"$-1\r\n"`)
   - Bulk Strings with embedded `"\r\n"`, to ensure the length is handled correctly: `b"$12\r\nhello\r\nworld\r\n"`
   - Invalid input (not starting with either `b"$"` or `b"+"`)
   - Very long Bulk Strings: Redis permits keys and values up to 512 MB. What is the largest data your code can handle?
     How long does it take to parse the data?
     
3. Use the techniques you learned so far in the workshop: 

   - Test your code's properties, not its implementation details
   - Use a tool like [Quickcheck](https://github.com/BurntSushi/quickcheck) or [Hypothesis](https://hypothesis.works)
     to generate your test data instead of hardcoding values
   
4. Once you have the above implemented, consider adding some of the following functionality (in increasing level of complexity):
   
   - RESP Errors (very similar to Simple Strings)
   - RESP Integers
   - RESP Arrays (may be nested!)
