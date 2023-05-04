use std::env;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::process::exit;

pub fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("need to pass an address to connect to");
        exit(1);
    }
    let host = args[1].clone();
    let port = if args.len() < 3 {
        80
    } else {
        args[2].parse().unwrap()
    };

    let socket_addrs: Vec<SocketAddr> = (host.clone() + ":" + &port.to_string())
        .to_socket_addrs()?
        .collect();

    let mut stream = TcpStream::connect(socket_addrs[0])?;
    let request = format!("GET / HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes())?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    let response = String::from_utf8_lossy(&buf);
    println!("Got response:\n{response}");

    Ok(())
}
