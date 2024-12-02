use std::io::{Read, Write};

use clap::{Parser, CommandFactory};
use urlencoding::{Encoded, decode_binary};

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

    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    if args.encode {
        let mut input = String::new();
        stdin.read_to_string(&mut input)?;
        let encoded = Encoded::new(&input);
        encoded.write(&mut stdout)?;
    } else {
        let mut input = Vec::new();
        stdin.read_to_end(&mut input)?;
        stdout.write_all(decode_binary(&input).as_ref())?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = urlcodec() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
