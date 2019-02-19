use std::io::stdin;
use std::io::Read;

// Read bytes (u8) from Stdin.
fn read_raw_bytes_from_stdin() -> Result<Vec<u8>, String> {
    let mut input: Vec<u8> = Vec::new();

    println!(
        "If no stdin input was provided, this program will 'hang'. \
         That is, no termination signal has been received by the program. \
         To continue, send a termination signal through the console (usually Ctrl+D)."
    );

    // If stdin is empty and no input file is defined, the programs waits* for
    // input (until NUL or some other termination signal byte?) instead of
    // returning Err(...).
    // The program has to be killed manually.
    //
    // * := (I think/assume)
    //
    // using stdin().bytes().collect::Vec<u8>() gives the same result.
    // locking first with stdin().lock() idem.
    let size = stdin().read_to_end(&mut input).map_err(|e| e.to_string())?;

    if size == 0 {
        Err("Stdin was empty".to_string())
    } else {
        println!("We got some bytes from stdin!, len={}!", size);
        Ok(input)
    }
}

// Succeeds on play.rust-lang.org
// But hangs on my local PC.
//
// Specifically, I would like to display a help page to a user when the stdin is empty
// on the program start.
// Since the console seems to allows text input, I think the stdin stream is 'open' when no
// input is piped to the application (Possibly no termination signal byte could be found (?)).
// If that is true I'm not sure how to close it;
//
// One idea is to run read_raw_bytes_from_stdin() in a separate thread with a maximum
// duration and hope that all data has already been received?
// Async could work too, but I would prefer to stay on stable.
//
//
//
#[test]
fn expect_empty_result() {
    let case = read_raw_bytes_from_stdin();

    assert!(case.is_err());
}

fn main() -> Result<(), String> {
    read_raw_bytes_from_stdin().map(|buffer| {
        println!("input size (bytes): {}", buffer.len());
        ()
    })
}
