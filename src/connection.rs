use std::{
    io::{Read, Write},
    net::TcpStream,
    time::{Duration, Instant},
};

use crate::request::Request;

// Relavent info about a result
pub struct ConnectionResult {
    rtt: Duration,
}

impl ConnectionResult {
    fn new(end: Duration) -> Self {
        Self { rtt: end }
    }
}

pub struct RepeatedResult {
    pub times: Vec<Duration>,
}

impl RepeatedResult {
    fn add_time(&mut self, rtt: Duration) {
        self.times.push(rtt);
    }
}

//TODO: stop ignoring errors
pub fn repeat_connection(
    repeat_for: usize,
    request: &Request,
    hostport: &str,
) -> Result<RepeatedResult, std::io::Error> {
    let mut output = RepeatedResult { times: vec![] };
    for _ in 0..repeat_for {
        if let Ok(time) = time_connection(request, hostport) {
            output.add_time(time.rtt);
        }
    }
    Ok(output)
}

pub fn time_connection(
    request: &Request,
    hostport: &str,
) -> Result<ConnectionResult, std::io::Error> {
    let start = Instant::now();

    let mut stream = TcpStream::connect(hostport)?;
    stream.write(request.to_string().as_bytes())?;
    let response_buffer = &mut String::new();
    stream.read_to_string(response_buffer)?;

    let end = start.elapsed();
    Ok(ConnectionResult::new(end))
}
