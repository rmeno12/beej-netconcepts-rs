use std::{
    io::Read,
    net::TcpStream,
    time::{Duration, SystemTime},
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("time.nist.gov:37")?;
    let mut buf: [u8; 4] = [0, 0, 0, 0];
    stream.read_exact(&mut buf)?;
    let ntp = Duration::new(u32::from_be_bytes(buf).into(), 0);
    let offset = Duration::new(2208988800, 0);
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let now_ntp = now + offset;

    println!("NTP time:\t{ntp:?}\nSystem time:\t{now_ntp:?}");

    Ok(())
}
