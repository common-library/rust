use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;

/// Client is object that provides client infomation.
pub struct Client {
    stream: Option<TcpStream>,
}

impl Client {
    /// new() returns a new Client object.
    ///
    /// # Examples
    /// ```
    /// use common_library::socket::client::Client;
    ///
    /// let mut client = Client::new();
    /// ```
    pub fn new() -> Client {
        Client { stream: None }
    }

    /// connect() is connect to the address.
    ///
    /// # Examples
    /// ```
    /// use common_library::socket::client::Client;
    /// use socket_server_mocker::server_mocker::ServerMocker;
    /// use socket_server_mocker::tcp_server_mocker::TcpServerMocker;
    /// use std::net::SocketAddr;
    /// use std::net::ToSocketAddrs;
    /// use std::time::Duration;
    ///
    /// let http_server_mocker = TcpServerMocker::new(0).unwrap();
    ///
    /// let mut client = Client::new();
    ///
    /// match client.connect(
    ///     SocketAddr::from(([127, 0, 0, 1], http_server_mocker.listening_port())),
    ///     Duration::new(3, 0),
    /// ) {
    ///     Ok(_) => (),
    ///     Err(e) => assert!(false, "{}", e),
    /// };
    /// ```
    pub fn connect(&mut self, address: SocketAddr, timeout: Duration) -> Result<(), String> {
        match TcpStream::connect_timeout(&address, timeout) {
            Ok(stream) => {
                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    /// read() is read data.
    /// # Examples
    /// ```
    /// use common_library::socket::client::Client;
    /// use socket_server_mocker::server_mocker::ServerMocker;
    /// use socket_server_mocker::server_mocker_instruction::ServerMockerInstruction;
    /// use socket_server_mocker::tcp_server_mocker::TcpServerMocker;
    /// use std::net::SocketAddr;
    /// use std::net::ToSocketAddrs;
    /// use std::time::Duration;
    ///
    /// let http_server_mocker = TcpServerMocker::new(0).unwrap();
    /// http_server_mocker.add_mock_instructions(&[
    ///     ServerMockerInstruction::ReceiveMessage,
    ///     ServerMockerInstruction::SendMessage("greeting\r\n".as_bytes().to_vec()),
    ///     ServerMockerInstruction::StopExchange,
    /// ]);
    ///
    /// let mut client = Client::new();
    ///
    /// match client.connect(
    ///     SocketAddr::from(([127, 0, 0, 1], http_server_mocker.listening_port())),
    ///     Duration::new(3, 0),
    /// ) {
    ///     Ok(_) => (),
    ///     Err(e) => assert!(false, "{}", e),
    /// };
    ///
    /// match client.read(1024) {
    ///     Ok(data) => assert_eq!(data, "greeting\r\n"),
    ///     Err(e) => assert!(false, "{}", e),
    /// };
    /// ```
    pub fn read(&mut self, receive_size: usize) -> Result<String, String> {
        let mut buffer = vec![0; receive_size];

        if let Some(stream) = &mut self.stream {
            match stream.read(&mut buffer) {
                Ok(_) => Ok(String::from(
                    String::from_utf8_lossy(&buffer.to_vec()).trim_end_matches(char::from(0)),
                )),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err(String::from("please call the connect function first"))
        }
    }

    /// write() is write data.
    /// # Examples
    /// ```
    /// use common_library::socket::client::Client;
    /// use socket_server_mocker::server_mocker::ServerMocker;
    /// use socket_server_mocker::server_mocker_instruction::ServerMockerInstruction;
    /// use socket_server_mocker::tcp_server_mocker::TcpServerMocker;
    /// use std::net::SocketAddr;
    /// use std::net::ToSocketAddrs;
    /// use std::time::Duration;
    ///
    /// let http_server_mocker = TcpServerMocker::new(0).unwrap();
    /// http_server_mocker.add_mock_instructions(&[
    ///     ServerMockerInstruction::ReceiveMessage,
    ///     ServerMockerInstruction::SendMessage("".into()),
    ///     ServerMockerInstruction::StopExchange,
    /// ]);
    ///
    /// let mut client = Client::new();
    ///
    /// match client.connect(
    ///     SocketAddr::from(([127, 0, 0, 1], http_server_mocker.listening_port())),
    ///     Duration::new(3, 0),
    /// ) {
    ///     Ok(_) => (),
    ///     Err(e) => assert!(false, "{}", e),
    /// };
    ///
    /// let data = String::from("test\r\n");
    /// match client.write(&data) {
    ///     Ok(_) => (),
    ///     Err(e) => assert!(false, "{}", e),
    /// };
    ///
    /// assert_eq!(std::str::from_utf8(&*http_server_mocker.pop_received_message().unwrap()).unwrap(), data)
    pub fn write(&mut self, data: &String) -> Result<(), String> {
        if let Some(stream) = &mut self.stream {
            match stream.write(data.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err(String::from("please call the connect function first"))
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use crate::socket::server::Server;
    use rand::Rng;

    #[test]
    fn connect_test() {
        let mut server = Server::new();
        let port = rand::thread_rng().gen_range(11000..12000);

        let job = |mut stream: TcpStream| {
            match stream.write("--- greeting ---\r\n".as_bytes()) {
                Ok(_) => (),
                Err(e) => assert!(false, "{}", e),
            }

            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => (),
                Err(e) => assert!(false, "{}", e),
            }

            match stream.flush() {
                Ok(_) => (),
                Err(e) => assert!(false, "{}", e),
            }
        };

        match server.start(String::from("0.0.0.0:") + &port.to_string(), job) {
            Ok(_) => (),
            Err(e) => assert!(false, "{}", e),
        };

        {
            let mut client = Client::new();

            match client.connect(
                SocketAddr::from(([127, 0, 0, 1], port)),
                Duration::new(3, 0),
            ) {
                Ok(_) => (),
                Err(e) => assert!(false, "{}", e),
            };

            match client.read(1024) {
                Ok(data) => assert_eq!(data, "--- greeting ---\r\n"),
                Err(e) => assert!(false, "{}", e),
            };

            let data = String::from("test\r\n");
            match client.write(&data) {
                Ok(_) => (),
                Err(e) => assert!(false, "{}", e),
            };
        }

        server.stop();
    }

    #[test]
    fn read_test() {
        connect_test();
    }

    #[test]
    fn write_test() {
        connect_test();
    }
}
