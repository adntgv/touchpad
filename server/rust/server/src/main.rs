use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("192.168.100.6:7000")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 10];
        let (_amt, _src) = socket.recv_from(&mut buf)?;

       println!("{:?}", buf )
    } // the socket is closed here
    Ok(())
}