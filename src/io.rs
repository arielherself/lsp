/// IO wrapper.

use std::io::{self, Read};
use crate::error;

fn read_header(buf: &mut Vec<u8>) -> io::Result<()> {
    let mut next_bytes = [0u8; 1];
    const SEPERATOR: [u8; 4] = [b'\r', b'\n', b'\r', b'\n'];
    let mut stack = [0u8; 4];
    loop {
        io::stdin().read_exact(&mut next_bytes)?;
        buf.push(next_bytes[0]);
        for i in 0..3 {
            stack[i] = stack[i + 1];
        }
        stack[3] = next_bytes[0];
        if SEPERATOR == stack {
            buf.truncate(buf.len() - 4);
            return Ok(())
        }
    }
}

pub fn next_request() ->  Result<json::JsonValue, Box<dyn std::error::Error>> {
    let mut header = Vec::new();
    read_header(&mut header)?;
    let s = String::from_utf8(header)?;
    if let Some(sep) = s.find(" ") {
        let len = s[sep + 1 ..].parse::<usize>()?;
        let mut ret = vec![0u8; len];
        io::stdin().read_exact(&mut ret)?;
        return Ok(json::parse(String::from_utf8(ret)?.as_str())?);
    } else {
        return Err(Box::new(error::ParseError::new(&String::from("Cannot find token ' '"))))
    }
}
