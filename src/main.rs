use std::{
    sync::{mpsc, Arc},
    thread::{self, sleep},
    time::Duration,
};

mod parse;
use parse::parse_url;
mod connection;
use connection::repeat_connection;
use request::RequestBuilder;
mod request;

fn main() -> std::io::Result<()> {
    // Initialization variables
    let action = "GET";
    let num_requests = 100;
    let num_threads = 25;
    let ramp_up_time_secs = 15;
    let timeout_secs = 1; // timeout begins after all threads were created

    // Process input
    let time_between =
        Duration::from_millis((ramp_up_time_secs as f64 / num_requests as f64 * 1000.) as u64);
    let timeout = Duration::from_secs(timeout_secs);

    let requests_per_thread = num_requests / num_threads;

    let Ok(url) = parse_url("http://eu.httpbin.org/get") else {
        panic!("The url was not valid");
    };

    let hostport = format!("{}:{}", &url.hostname, url.port);
    let request = RequestBuilder::empty()
        .with_action(&action)
        .with_protocol(&url.protocol)
        .with_path(&url.path)
        .with_hostname(&url.hostname)
        .with_port(url.port)
        .build()
        .unwrap(); // TODO: handle error
    let thread_state = Arc::new((hostport, request));

    let (producer, consumer) = mpsc::channel();

    // Thread creation for test
    let mut errors_in_threads = 0;
    let mut handlers = vec![];
    for i in 0..num_threads {
        // clone so we can use in each thread
        let state = Arc::clone(&thread_state);
        let producer = producer.clone();
        let builder = thread::Builder::new();

        // if there is a ramp up time set
        if ramp_up_time_secs > 1 {
            sleep(time_between);
        }
        match builder.spawn(move || {
            // Thread Logic
            if let Ok(result) = repeat_connection(requests_per_thread, &state.1, &state.0) {
                for time in result.times {
                    let _ = producer.send(time);
                }
            }
        }) {
            Ok(handler) => handlers.push((handler, i)),
            Err(e) => {
                println!("{} thread could not be created. Error reported {}", i, e);
                errors_in_threads += 1;
            }
        }
    }
    println!(">>> All threads were created");

    // wait for threads to finish
    for (handler, i) in handlers {
        if let Err(e) = handler.join() {
            println!("{} had an error {:?}", i, e);
        }
    }

    // read all input from threads
    let mut waiting_for = num_requests;
    let mut rr_times = vec![];
    while let Ok(recieved) = consumer.recv_timeout(timeout) {
        rr_times.push(recieved.as_millis());
        waiting_for -= 1;

        if waiting_for - errors_in_threads <= 0 {
            break;
        }
    }

    // stats
    let actual = rr_times.len() as u128;
    let average = rr_times.iter().fold(0, |acc, r| acc + r) / actual;
    let max = match rr_times.iter().max() {
        Some(max) => max.to_string(),
        None => "<No Max Found>".to_string(),
    };
    let min = match rr_times.iter().min() {
        Some(min) => min.to_string(),
        None => "<No Min Found>".to_string(),
    };
    println!("Time Between Threads: {time_between:?}");
    println!("Errors during creation: {}", errors_in_threads);
    println!(
        "Actual/Expected {actual}/{num_requests} | Avg: {average} ms | Min: {min} ms | Max: {max} ms"
    );

    Ok(())
}
