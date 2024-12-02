mod urlcodec;
use urlcodec::*;

use std::collections::HashSet;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, help="urldecode")]
    decode: bool,
}

fn urlcodec() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let preserve: HashSet<char> = ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .chain(":/?,-.".chars())
        .collect();

    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    if args.decode {
        url_decode_stream(&mut stdin, &mut stdout)?;
    } else {
        url_encode_stream(preserve, &mut stdin, &mut stdout)?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = urlcodec() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
