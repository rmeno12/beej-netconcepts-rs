use std::{
    env,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
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
        let lines: Vec<&str> = req.split("\r\n").collect();
        let path_str = &lines[0].split(' ').collect::<Vec<&str>>()[1];
        let path = Path::new(path_str);
        let clean_path: &Path;
        if let Some(fname) = path.file_name() {
            clean_path = Path::new(fname);
        } else {
            continue;
        }
        let response = if let Ok(data) = std::fs::read_to_string(clean_path) {
            let mime_type = match clean_path.extension() {
                Some(ext) if ext.to_string_lossy() == "txt" => "text/plain",
                Some(ext) if ext.to_string_lossy() == "html" => "text/html",
                _ => "",
            };
            let r = format!("HTTP/1.1 200 OK\r\nContent-Type: {mime_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{data}", data.len());
            r.as_bytes().to_owned()
        } else {
            // send 404
            let r = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 13\r\nConnection: close\r\n\r\n404 not found".to_string();
            r.as_bytes().to_owned()
        };
        stream.write_all(&response)?;
    }
}
