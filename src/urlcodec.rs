use std::collections::HashSet;
use std::io::{Read, Write, BufReader, BufWriter};

use character_stream::CharacterStream;

pub fn url_encode_stream<R: Read, W: Write>(preserve: HashSet<char>, r: &mut R, w: &mut W) ->
    Result<(), Box<dyn std::error::Error>>
{
    let input = CharacterStream::new(BufReader::new(r), false);
    let mut output = BufWriter::new(w);
    let mut in_buf = [0u8; 4];
    let mut out_buf = [0u8; 3];
    out_buf[0] = b'%';
    for c in input {
        let c = c?;
        let bytes = c.encode_utf8(&mut in_buf);
        let nbytes = bytes.len();
        let in_bytes = &in_buf[..nbytes];
        if preserve.contains(&c) {
            output.write_all(in_bytes)?;
        } else {
            let digit = |d| char::from_digit(d as u32, 16).unwrap().to_ascii_uppercase() as u8;
            for &b in in_bytes {
                out_buf[1] = digit((b >> 4) & 0xf);
                out_buf[2] = digit(b & 0xf);
                output.write_all(&out_buf)?;
            }
        }
    }
    Ok(())
}

#[allow(unused)]
pub fn url_decode_stream<R: Read, W: Write>(r: &mut R, w: &mut W) ->
    Result<(), Box<dyn std::error::Error>>
{
    enum State {
        Run,
        Percent1,
        Percent2,
    }

    let input = CharacterStream::new(BufReader::new(r), false);
    let mut output = BufWriter::new(w);
    let mut in_buf = [0u8; 4];
    let mut state = State::Run;
    let mut out_byte = 0u8;
    for c in input {
        let c = c?;
        match state {
            State::Run => {
                match c {
                    '%' => {
                        state = State::Percent1;
                    }
                    _ => {
                        let bytes = c.encode_utf8(&mut in_buf);
                        let nbytes = bytes.len();
                        let in_bytes = &in_buf[..nbytes];
                        output.write_all(in_bytes)?;
                    }
                }
            }
            State::Percent1 => {
                match c {
                    '%' => {
                        output.write_all(b"%")?;
                        state = State::Run;
                    }
                    d if d.is_ascii_hexdigit() => {
                        out_byte = (d.to_digit(16).unwrap() as u8) << 4;
                        state = State::Percent2;
                    }
                    _ => {
                        return Err(format!("bad percent encoding %{:?}", c).into());
                    }
                }
            }
            State::Percent2 => {
                match c {
                    d if d.is_ascii_hexdigit() => {
                        out_byte |= d.to_digit(16).unwrap() as u8;
                        output.write_all(&[out_byte])?;
                        state = State::Run;
                    }
                    _ => {
                        return Err(format!("bad percent encoding %x{:?}", c).into());
                    }
                }
            }
        }
    }
    Ok(())
}
