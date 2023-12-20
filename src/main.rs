use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::path::Path;
use std::{env, process, error};

const HYPR_SOCKET: &str = "HYPRLAND_INSTANCE_SIGNATURE";
const SOCKET_NAME: &str = ".socket2.sock";

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    match try_main() {
        Ok(()) => return,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }
}

fn try_main() -> Result<()> {
    let hypr_socket = env::var(HYPR_SOCKET).unwrap_or_else(|x| {
        eprintln!("Env variable {} not found: {x}", HYPR_SOCKET);
        process::exit(1);
    });
    let sock_path = Path::new("/tmp/hypr/").join(hypr_socket).join(SOCKET_NAME);

    let mut stream = UnixStream::connect(sock_path)?;
    println!("Connected to hyprland socket.");
    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        stream.read(&mut buf)?;
        if let Some(end) = newline_idx(&buf) {
            let msg = std::str::from_utf8(&buf)?;
            let msg = &msg[..end];
            println!("msg: {msg}");
        }
    }
}

fn newline_idx(arr: &[u8]) -> Option<usize> {
    for (i, b) in arr.iter().enumerate() {
        if *b == 10 as u8 {
            return Some(i);
        }
    }
    None
}