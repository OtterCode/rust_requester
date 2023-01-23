use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{ Read, Write };
use crate::configuration::port::Port;
use crate::error::Error;

// Create the raw tcp listener
pub fn raw_tcp_listener(port: Port) -> Result<String, Error> {
    let address = SocketAddr::from(([127, 0, 0, 1], port.as_u16()));
    let listener: TcpListener = TcpListener::bind(address)?;
    let code = listener
        .incoming()
        .filter_map(|stream| {
            stream.ok().and_then(|stream| {
                let code = collect_stream(stream);
                code
            })
        }).next();
    
    code.ok_or(Error::AuthServerClosedEarly)
}

fn collect_stream(mut stream: TcpStream) -> Option<String> {
    // The odds of a request url being larger than 2kb is slim. 
    // This would be risky in production.
    let mut buffer = [0; 2048]; 
    stream.read(&mut buffer).unwrap();
    let raw_request = String::from_utf8_lossy(&buffer[..]);
    
    let code = extract_code(raw_request.to_string());
    
    if code.is_some() {
        stream.write("HTTP/1.1 200 OK\r\n\r\n200 OK".as_bytes()).unwrap();
    } else {
        stream.write("HTTP/1.1 401 Unauthorized\r\n\r\n401 Unauthorized".as_bytes()).unwrap();
    }

    code
}

fn extract_code(raw_request: String) -> Option<String> {
    let code_and_tail = raw_request.rsplit_once("code=")?.1;
    let code_and_url = code_and_tail.rsplit_once(" HTTP/1.1")?.0;
    let code = code_and_url
        .split('&')
        .next()?
        .to_string();

    if code.len() > 0 {
        Some(code)
    } else {
        None
    }
}
