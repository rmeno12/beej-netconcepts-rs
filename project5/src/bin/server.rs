use std::{
    env,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    process::exit,
};

fn read_to_sequence(stream: &mut TcpStream, sequence: &str) -> Result<String, std::io::Error> {
    let mut reader = BufReader::new(stream);
    let mut buf = String::new();
    let mut result = String::new();

    loop {
        let bytes = reader.read_line(&mut buf)?;
        if bytes == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "End of stream reached",
            ));
        }
        result.push_str(&buf);

        if buf.contains(sequence) {
            return Ok(result);
        }

        buf.clear();
    }
}

pub fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("need to pass a port to bind to");
        exit(1);
    }
    let port: i32 = args[1].parse().expect("couldn't parse port");

    let socket = TcpListener::bind(format!("127.0.0.1:{port}"))?;

    loop {
        let (mut stream, _) = socket.accept()?;
        // do things with the stream
        let req = read_to_sequence(&mut stream, "\r\n")?;
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 6\r\nConnection: close\r\n\r\nHello!";
        stream.write_all(response.as_bytes())?;
        println!("{req}");
    }
}
