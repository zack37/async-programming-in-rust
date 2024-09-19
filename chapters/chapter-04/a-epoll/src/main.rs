use a_epoll::{
    ffi::{self, Event},
    poll::Poll,
};
use std::{
    collections::HashSet,
    io::{self, Read, Result, Write},
    net::TcpStream,
};

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
         Host: localhost\r\n\
         Connection: close\r\n\
         \r\n"
    )
}

fn handle_events(
    events: &[Event],
    streams: &mut [TcpStream],
    handled: &mut HashSet<usize>,
) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 4096];

        loop {
            match streams[index].read(&mut data) {
                Ok(n) if n == 0 => {
                    if !handled.insert(index) {
                        break;
                    }
                    handled_events += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);

                    println!("RECEIVED: {event:?}");
                    println!("{txt}\n------\n");
                }
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::Interrupted =>
                {
                    break
                }
                Err(e) => return Err(e),
            }
        }
    }

    Ok(handled_events)
}

fn main() -> Result<()> {
    let mut poll = Poll::new()?;
    let n_events = 5;
    let mut streams = vec![];
    let addr = "localhost:8080";

    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let mut stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;
        stream.write_all(request.as_bytes())?;
        poll.registry()
            .register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;
        streams.push(stream);
    }

    let mut handled_ids = HashSet::new();

    let mut handled_events = 0;
    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;
        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }
        handled_events += handle_events(&events, &mut streams, &mut handled_ids)?;
    }

    println!("FINISHED");
    Ok(())
}
