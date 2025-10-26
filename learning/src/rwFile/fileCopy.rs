/*
文件复制
cargo run source.txt destination.txt
*/

use std::env;
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <source> <destination>", args[0]);
        return Ok(());
    }

    fs::copy(&args[1], &args[2])?;
    Ok(())
}

