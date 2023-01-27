use crate::configuration::port::Port;
use crate::error::Error;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use tokio::sync::mpsc::{error::TryRecvError, Receiver};
use tokio::time::{sleep, Duration};

const POLL_DELAY_NS: u32 = 50_000u32;

// Create the raw tcp listener. This was originally a very clever little thing
// that simply ran a filter_map on all streams and called .next once.
// Unfortunately, there was no reasonable way to stop the stream midway through
// in case of a cancel, which is called for in the GUI version. Tokio was
// panicking and throwing a fit when the listeners weren't closed properly
// before starting another.
//
// I also could have used a oneshot, or a warp server with graceful shutdown,
// or any number of other async solutions but that's not really the goal of
// this app. If I wanted to do things as quickly as possible, 99% of this app
// wouldn't exist, and it would just ingest a set of JSON credentials. So,
// here also I went with a slightly less pragmatic, lower level, method. This
// is a showpiece after all.
//
// My one concession is using tokio to run the sleep. No sense in getting
// sloppy with resources just because we're indulging in primitive code.
pub async fn raw_tcp_listener(port: Port, mut signal: Receiver<()>) -> Result<String, Error> {
    let address = SocketAddr::from(([127, 0, 0, 1], port.as_u16()));
    let listener: TcpListener = TcpListener::bind(address)?;
    listener.set_nonblocking(true)?;
    for maybe_stream in listener.incoming() {
        match maybe_stream {
            Ok(stream) => {
                if let Ok(code) = collect_stream(stream) {
                    return Ok(code);
                };
            }
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                match signal.try_recv() {
                    Ok(_) => {
                        return Err(Error::AuthServerClosedEarly);
                    }
                    Err(err) if err == TryRecvError::Empty => {
                        sleep(Duration::new(0, POLL_DELAY_NS)).await;
                    }
                    Err(_) => {
                        return Err(Error::AuthServerClosedEarly);
                    }
                }
            }
            Err(_) => {
                return Err(Error::AuthServerClosedEarly);
            }
        };
    }

    Err(Error::AuthServerClosedEarly)
}

fn collect_stream(mut stream: TcpStream) -> Result<String, Error> {
    // The odds of a request url being larger than 2kb is slim.
    // This would be risky in production, but we're just going
    // to pull in up to 3kib and process them as a string.
    let mut buffer = [0; 3072];
    stream.read(&mut buffer)?;
    let raw_request = String::from_utf8_lossy(&buffer[..]);

    let code = extract_code(raw_request.to_string());

    if code.is_some() {
        stream
            .write("HTTP/1.1 200 OK\r\n\r\n200 OK".as_bytes())
            .unwrap();
    } else {
        stream
            .write("HTTP/1.1 401 Unauthorized\r\n\r\n401 Unauthorized".as_bytes())
            .unwrap();
    }

    code.ok_or(Error::MissingToken)
}

fn extract_code(raw_request: String) -> Option<String> {
    let code_and_tail = raw_request.rsplit_once("code=")?.1;
    let code_and_url = code_and_tail.rsplit_once(" HTTP/1.1")?.0;
    let code = code_and_url.split('&').next()?.to_string();

    if code.len() > 0 {
        Some(code)
    } else {
        None
    }
}
