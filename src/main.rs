use std::io::stdin;
use std::io::Read;

fn read_raw_bytes_from_stdin() -> Result<Vec<u8>, String> {
    let mut input: Vec<u8> = Vec::new();

    // If stdin is empty and no input file is defined, the programs waits for input instead of returning the Err(...)
    // The program has to be killed manually.
    //
    // using stdin().bytes().collect::Vec<u8>() has the same result.
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
#[test]
fn expect_empty_result() {
    let case = read_raw_bytes_from_stdin();

    assert!(case.is_err());
}


fn main() {
    println!("Hello, world!");
}
