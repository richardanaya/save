use colored::*;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut buffer = Vec::new();

    if args.len() < 2 {
        println!("Error: {}","first arg should be a file: save <foo.text>".red());
        return Ok(());
    }
    
    for i in io::stdin().lock().bytes() {
        buffer.push(i?);
    }

    let mut f = File::create(&args[1])?;
    f.write_all(&buffer)?;
    Ok(())
}
