use crate::http::{ParseError, Request, Response, StatusCode}; // crate is the root of the project
use std::convert::TryFrom;
use std::io::{Write, Read};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    // this is a default implementation, can be overwritten
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

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
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) { // equivalent to &buffer as &[u8]
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
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
