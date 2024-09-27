use crate::http::Request; // crate is the root of the project
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    // the address is not known at compile time so we have to use String
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    // to use attributes from a struct in a method, you have to use self as the first parameter
    // the function will take ownership of the struct, so the struct will be deallocated
    // when the function ends, if you want to prevent that, you should use a reference or mutable reference
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) { // equivalent to &buffer as &[u8]
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => {println!("Failed to establish a connection: {}", e)},
            }
        }

        // label loops
        // 'outer: loop {
        //     loop {
        //         break 'outer; --> this breaks the outer loop from the inner one
        //     }
        // }
    }
}
