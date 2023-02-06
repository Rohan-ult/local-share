use std::io::{ErrorKind, Result};
use std::net::{IpAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

pub fn local_ip() -> IpAddr {
    local_ip_address::local_ip().expect("Local IP address not available")
}

pub fn discover(port: u16, duration: Duration, cb: impl Fn(IpAddr, &[u8])) -> Result<()> {
    let socket = UdpSocket::bind((local_ip(), 0))?;

    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(duration))?;
    socket.send_to(b"Hello!", ("255.255.255.255", port))?;

    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => cb(src.ip(), &buf[..amt]),
            Err(err) if err.kind() == ErrorKind::TimedOut => return Ok(()),
            Err(err) => return Err(err),
        }
    }
}

pub fn announce<A: ToSocketAddrs>(addr: A, data: impl AsRef<[u8]>) -> Result<()> {
    let socket = UdpSocket::bind(addr)?;
    let mut buf = [0; 1024];
    loop {
        let (_, src) = socket.recv_from(&mut buf)?;
        socket.send_to(data.as_ref(), src)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_discover_peer() -> Result<()> {
        thread::spawn(|| announce("0.0.0.0:9876", "HI!"));
        thread::spawn(|| announce((local_ip(), 9876), "HI!"));
        thread::sleep(Duration::from_secs(1));

        discover(9876, Duration::from_secs(3), |ip, msg| {
            println!("IP: {ip}");
            assert_eq!(msg, b"HI!");
        })
    }
}