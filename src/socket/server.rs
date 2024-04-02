use std::io;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

/// Server is object that provides server infomation.
pub struct Server {
    condition: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    local_address: Option<SocketAddr>,
}

impl Server {
    /// new() returns a new Server object.
    ///
    /// # Examples
    /// ```
    /// use common_library::socket::server::Server;
    ///
    /// let mut server = Server::new();
    /// ```
    pub fn new() -> Server {
        Server {
            condition: Arc::new(AtomicBool::new(true)),
            handle: None,
            local_address: None,
        }
    }

    /// Start() is start the server.
    ///
    /// # Examples
    /// ```
    /// use common_library::socket::server::Server;
    /// use rand::Rng;
    /// use std::io::prelude::*;
    /// use std::net::TcpStream;
    ///
    /// let mut server = Server::new();
    /// let port = rand::thread_rng().gen_range(10000..11000).to_string();
    ///
    /// let job = |mut stream: TcpStream| {
    ///     match stream.write("--- greeting ---\r\n".as_bytes()) {
    ///         Ok(_) => (),
    ///         Err(e) => assert!(false, "{}", e),
    ///     }
    ///
    ///     let mut buffer = [0; 1024];
    ///     match stream.read(&mut buffer) {
    ///         Ok(_) => (),
    ///         Err(e) => assert!(false, "{}", e),
    ///     }
    ///
    ///     match stream.write(&buffer) {
    ///         Ok(_) => (),
    ///         Err(e) => assert!(false, "{}", e),
    ///     }
    ///
    ///     match stream.flush() {
    ///         Ok(_) => (),
    ///         Err(e) => assert!(false, "{}", e),
    ///     }
    /// };
    ///
    /// match server.start(String::from("0.0.0.0:") + &port, job) {
    ///     Ok(_) => (),
    ///     Err(e) => assert!(false, "{}", e),
    /// };
    ///
    /// {
    ///     match TcpStream::connect(String::from("localhost:") + &port) {
    ///         Ok(mut stream) => {
    ///             let mut buffer = [0; 1024];
    ///             match stream.read(&mut buffer) {
    ///                 Ok(_) => (),
    ///                 Err(e) => assert!(false, "{}", e),
    ///             }
    ///             assert_eq!(
    ///                 String::from_utf8_lossy(&buffer.to_vec()).trim_end_matches(char::from(0)),
    ///                 "--- greeting ---\r\n"
    ///             );
    ///
    ///             let data = "test\r\n";
    ///             match stream.write(data.as_bytes()) {
    ///                 Ok(_) => (),
    ///                 Err(e) => assert!(false, "{}", e),
    ///             }
    ///
    ///             match stream.read(&mut buffer) {
    ///                 Ok(_) => (),
    ///                 Err(e) => assert!(false, "{}", e),
    ///             }
    ///             assert_eq!(
    ///                 String::from_utf8_lossy(&buffer.to_vec()).trim_end_matches(char::from(0)),
    ///                 data
    ///             );
    ///         }
    ///         Err(e) => assert!(false, "{}", e),
    ///     }
    /// }
    ///
    /// server.stop();
    /// ```
    pub fn start<T>(&mut self, address: T, job: fn(TcpStream)) -> Result<(), io::Error>
    where
        T: ToSocketAddrs,
    {
        let listener = TcpListener::bind(address)?;

        self.local_address = Some(listener.local_addr()?);

        self.condition.store(false, Ordering::Relaxed);
        let condition_clone = self.condition.clone();

        self.handle = Some(thread::spawn(move || {
            for stream in listener.incoming() {
                if condition_clone.load(Ordering::Relaxed) {
                    break;
                }

                match stream {
                    Ok(stream) => job(stream),
                    Err(e) => println!("{}", e),
                }
            }
        }));

        Ok(())
    }

    /// Stop is stop the server.
    ///
    /// # Examples
    /// ```
    /// use common_library::socket::server::Server;
    ///
    /// let server = Server::new();
    ///
    /// server.stop();
    /// ```
    pub fn stop(self) {
        if self.condition.load(Ordering::Relaxed) {
            return;
        }

        if let Some(local_address) = self.local_address {
            self.condition.store(true, Ordering::Relaxed);
            let _ = TcpStream::connect(local_address);

            if let Some(handle) = self.handle {
                if handle.is_finished() == false {
                    let _ = handle.join();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use rand::Rng;

    #[test]
    fn start_test() {
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

            match stream.write(&buffer) {
                Ok(_) => (),
                Err(e) => assert!(false, "{}", e),
            }

            match stream.flush() {
                Ok(_) => (),
                Err(e) => assert!(false, "{}", e),
            }
        };

        let mut server = Server::new();
        let port = rand::thread_rng().gen_range(10000..11000).to_string();

        match server.start(String::from("0.0.0.0:") + &port, job) {
            Ok(_) => (),
            Err(e) => assert!(false, "{}", e),
        };

        {
            match TcpStream::connect(String::from("localhost:") + &port) {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => (),
                        Err(e) => assert!(false, "{}", e),
                    }
                    assert_eq!(
                        String::from_utf8_lossy(&buffer.to_vec()).trim_end_matches(char::from(0)),
                        "--- greeting ---\r\n"
                    );

                    let data = "test\r\n";
                    match stream.write(data.as_bytes()) {
                        Ok(_) => (),
                        Err(e) => assert!(false, "{}", e),
                    }

                    match stream.read(&mut buffer) {
                        Ok(_) => (),
                        Err(e) => assert!(false, "{}", e),
                    }
                    assert_eq!(
                        String::from_utf8_lossy(&buffer.to_vec()).trim_end_matches(char::from(0)),
                        data
                    );
                }
                Err(e) => assert!(false, "{}", e),
            }
        }

        server.stop();
    }

    #[test]
    fn stop_test() {
        let server = Server::new();

        server.stop();
    }
}
