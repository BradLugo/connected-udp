use std::convert::TryFrom;
use std::io;
use std::net::{SocketAddr, UdpSocket};

/// A connected UDP socket.
///
/// Essentially a wrapper around [`std::net::UdpSocket`] and [`std::net::SocketAddr`]
/// that provides a safer and consistent API for connected UDP sockets.
///
/// # Examples
///
/// ## Have `connected-udp` connect the socket
///
/// ```
/// use connected_udp::ConnectedUdpSocket;
/// use std::net::UdpSocket;
///
/// fn main() -> std::io::Result<()> {
///     // Set up host UDP socket
///     let host = UdpSocket::bind("127.0.0.1:0")?;
///     let host_addr = host.local_addr()?;
///
///     // Set up connected client socket
///     let client = UdpSocket::bind("127.0.0.1:0")?;
///     let conn_client = ConnectedUdpSocket::connect(client, host_addr)?;
///
///     // Send datagram
///     host.send_to("ping".as_bytes(), conn_client.local_addr()?)?;
///
///     // Receive datagram
///     let mut buf = [0; 32];
///     let n = conn_client.recv(&mut buf)?;
///
///     // Send reversed datagram back
///     let buf = &mut buf[..n];
///     buf.reverse();
///     conn_client.send(buf)?;
///
///     Ok(())
/// }
/// ```
///
/// ## Convert a `std::net::UdpSocket` that's already been connected
///
/// ```
/// use connected_udp::ConnectedUdpSocket;
/// use std::net::UdpSocket;
///
/// fn main() -> std::io::Result<()> {
///     // Set up host UDP socket
///     let host = UdpSocket::bind("127.0.0.1:0")?;
///     let host_addr = host.local_addr()?;
///
///     // Set up client UDP socket
///     let client = UdpSocket::bind("127.0.0.1:0")?;
///     client.connect(host_addr)?;
///
///     // Convert client UDP socket to connected socket
///     let conn_client = ConnectedUdpSocket::try_from(client)?;
///
///     // Send datagram
///     host.send_to("ping".as_bytes(), conn_client.local_addr()?)?;
///
///     // Receive datagram
///     let mut buf = [0; 32];
///     let n = conn_client.recv(&mut buf)?;
///
///     // Send reversed datagram back
///     let buf = &mut buf[..n];
///     buf.reverse();
///     conn_client.send(buf)?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct ConnectedUdpSocket {
    socket: UdpSocket,
    peer: SocketAddr,
}

impl ConnectedUdpSocket {
    /// Connects `socket` to the remote server specified in `peer`, setting the
    /// destination for `send` and limiting packets that are read via `recv` to
    /// that address.
    ///
    /// # Examples
    ///
    /// ```
    /// # use connected_udp::ConnectedUdpSocket;
    /// # use std::net::UdpSocket;
    /// # fn main() {
    ///  let host = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to host address");
    ///  let host_addr = host.local_addr().expect("couldn't retrieve host address");
    ///
    ///  let client = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to client address");
    ///  let conn_client = ConnectedUdpSocket::connect(client, host_addr).expect("couldn't client to host");
    /// # }
    /// ```
    pub fn connect(socket: UdpSocket, peer: SocketAddr) -> io::Result<Self> {
        socket.connect(peer)?;
        Ok(Self { socket, peer })
    }

    /// Returns the local socket address for this socket.
    ///
    /// # Examples
    /// ```
    /// # use connected_udp::ConnectedUdpSocket;
    /// # use std::net::UdpSocket;
    /// # fn main() {
    ///  let host = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to host address");
    ///  let host_addr = host.local_addr().expect("couldn't retrieve host address");
    ///
    ///  let client = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to client address");
    ///  let conn_client = ConnectedUdpSocket::connect(client, host_addr).expect("couldn't client to host");
    ///
    ///  let local_addr = conn_client.local_addr();
    ///  println!("local addr: {}", local_addr);
    /// # }
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    /// Returns the socket address of the remote peer.
    ///
    /// # Examples
    /// ```
    /// # use connected_udp::ConnectedUdpSocket;
    /// # use std::net::UdpSocket;
    /// # fn main() {
    ///  let host = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to host address");
    ///  let host_addr = host.local_addr().expect("couldn't retrieve host address");
    ///
    ///  let client = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to client address");
    ///  let conn_client = ConnectedUdpSocket::connect(client, host_addr).expect("couldn't client to host");
    ///
    ///  let peer_addr = conn_client.peer_addr();
    ///  println!("remote peer addr: {}", peer_addr);
    /// # }
    pub fn peer_addr(&self) -> SocketAddr {
        self.peer
    }

    /// Sends data through the underlying socket.
    /// # Examples
    ///
    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.socket.send(buf)
    }

    /// Receives data from the socket and writes it into the provided buffer.
    /// # Examples
    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv(buf)
    }
}

impl TryFrom<UdpSocket> for ConnectedUdpSocket {
    type Error = io::Error;

    fn try_from(socket: UdpSocket) -> Result<Self, Self::Error> {
        let peer = socket.peer_addr()?;
        Ok(Self { socket, peer })
    }
}

impl AsRef<UdpSocket> for ConnectedUdpSocket {
    fn as_ref(&self) -> &UdpSocket {
        &self.socket
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn try_from_not_connected_error() {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let err = ConnectedUdpSocket::try_from(socket).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotConnected);
    }

    #[test]
    fn try_from_connected() {
        let receiver = UdpSocket::bind("127.0.0.1:0").unwrap();
        let recv_addr = receiver.local_addr().unwrap();

        let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
        sender.connect(recv_addr).unwrap();

        let sender_conn = ConnectedUdpSocket::try_from(sender.try_clone().unwrap()).unwrap();
        assert_eq!(sender_conn.peer_addr(), recv_addr);

        let local = sender_conn.local_addr().unwrap();
        assert_eq!(
            local.ip(),
            std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
        );

        assert_eq!(
            sender_conn.local_addr().unwrap(),
            sender.local_addr().unwrap()
        );
    }

    #[test]
    fn connect_and_send_recv() {
        let receiver = UdpSocket::bind("127.0.0.1:0").unwrap();
        let recv_addr = receiver.local_addr().unwrap();

        let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
        let sender_conn = ConnectedUdpSocket::connect(sender, recv_addr).unwrap();

        let handle = thread::spawn(move || {
            let mut buf = [0u8; 32];
            let (n, from) = receiver.recv_from(&mut buf).unwrap();
            assert_eq!(&buf[..n], b"ping");
            from
        });

        let n = sender_conn.send(b"ping").unwrap();
        assert_eq!(n, 4);

        let from = handle.join().unwrap();
        assert_eq!(from, sender_conn.local_addr().unwrap());
    }

    #[test]
    fn as_ref_udp_socket() {
        let receiver = UdpSocket::bind("127.0.0.1:0").unwrap();
        let recv_addr = receiver.local_addr().unwrap();

        let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
        let sender_conn = ConnectedUdpSocket::connect(sender, recv_addr).unwrap();

        let raw: &UdpSocket = sender_conn.as_ref();

        assert_eq!(raw.peer_addr().unwrap(), recv_addr);
        assert_eq!(raw.local_addr().unwrap(), sender_conn.local_addr().unwrap());

        let handle = thread::spawn(move || {
            let mut buf = [0u8; 32];
            let (n, from) = receiver.recv_from(&mut buf).unwrap();
            assert_eq!(&buf[..n], b"asref");
            from
        });

        let n = raw.send(b"asref").unwrap();
        assert_eq!(n, 5);

        let from = handle.join().unwrap();
        assert_eq!(from, sender_conn.local_addr().unwrap());
    }
}
