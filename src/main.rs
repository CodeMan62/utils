use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Need a command");
        process::exit(0);
    }
    let cmd = &args[1];
    match cmd.as_str() {
        "cp" | "copy" => {
            let src = &args[2];
            let dst = &args[3];
            copy_file(src, dst).unwrap();
        }
        _ => {
            eprintln!("Unknown command");
        }
    }
}

#[allow(unused)]
fn copy_file(source: &str, destination: &str) -> std::io::Result<()> {
    let mut source_file = File::open(source)?;
    let mut dest_file = File::create(destination)?;

    let mut buffer = [0; 8192];
    let mut copied = 0u64;

    loop {
        let bytes_read = source_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        dest_file.write_all(&buffer[..bytes_read])?;
        copied += bytes_read as u64;
    }
    println!("Copied {} bytes", copied);
    Ok(())
}
