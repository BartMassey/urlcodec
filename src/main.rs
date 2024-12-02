mod urlcodec;
use urlcodec::*;

//use std::io::{Read, Write};
use std::collections::HashSet;

use clap::{Parser, CommandFactory};

#[derive(Parser)]
struct Args {
    #[arg(short, long, help="urlencode")]
    encode: bool,
    #[arg(short, long, help="urldecode")]
    decode: bool,
}

fn urlcodec() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if !(args.encode ^ args.decode) {
        return Err(Args::command().error(
            clap::error::ErrorKind::ArgumentConflict,
            "must specify either encode or decode (but not both)",
        ).into());
    }

    let preserve: HashSet<char> = ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .chain(":/?,-.".chars())
        .collect();

    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    if args.encode {
        url_encode_stream(preserve, &mut stdin, &mut stdout)?;
    } else {
        url_decode_stream(&mut stdin, &mut stdout)?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = urlcodec() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
