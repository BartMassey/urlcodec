mod urlcodec;
use urlcodec::*;

use std::collections::HashSet;
use std::io::{stdin, stdout, Cursor, Read, Write};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, help="urldecode")]
    decode: bool,
    #[arg(short, long, help="characters to preserve (always alphanumerics)", default_value=":/?,-.")]
    preserve: String,
    #[arg(short, long, help="streaming I/O (usually not needed)")]
    stream: bool,
}

fn urlcodec() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let preserve: HashSet<char> = ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .chain(args.preserve.chars())
        .collect();

    let mut stdin: Box<dyn Read> = if args.stream {
        Box::new(stdin().lock())
    } else {
        let mut input = String::new();
        stdin().lock().read_to_string(&mut input)?;
        Box::new(Cursor::new(input.trim().to_owned()))
    };
    let mut stdout = stdout().lock();
    if args.decode {
        url_decode_stream(&mut stdin, &mut stdout)?;
    } else {
        url_encode_stream(preserve, &mut stdin, &mut stdout)?;
    }
    if !args.stream {
        writeln!(stdout)?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = urlcodec() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
