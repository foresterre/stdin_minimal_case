use std::io::Read;

use futures::{Future, Stream, Sink};


// Probably will want to use tokio-io instead
fn something() -> Result<(), Box<std::error::Error>> {
    use futures::Stream;

    let s = tokio_stdin::spawn_stdin_stream_unbounded().wait();

    Ok(())
}

/// Read bytes (u8) from Stdin.
/// Returns:
/// - Ok(_) if the stdin contained any bytes.
/// - Err() if the stdin was empty.
fn read_raw_bytes_from_stdin_async() -> Result<Vec<u8>, String> {
    let mut input: Vec<u8> = Vec::new();

    println!("This is a work in progress minimal example to read the stdin asynchronously");

    something()
        .map_err(|e|e.to_string())
        .map(|_| vec![])
//    Err("in progress".to_string())
}

#[test]
fn expect_empty_result_async() {
    let case = read_raw_bytes_from_stdin_async();

    assert!(case.is_err());
}

fn main() -> Result<(), String> {
    read_raw_bytes_from_stdin_async().map(|buffer| {
        println!("input size (bytes): {}", buffer.len());
        ()
    })
}
