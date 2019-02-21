use std::env;
use std::time::Duration;

use futures::stream::{once, Stream};
use tokio_stdin::spawn_stdin_stream_unbounded;
use tokio_timer::{Timer, TimerError};

#[derive(Debug)]
enum Error {
    Timer(TimerError),
    Stdin(()),
}

enum Event {
    Byte(u8),
    Second,
    Done,
}

fn main() -> Result<(), String> {
    let seconds_stream = Timer::default()
        .interval(Duration::from_secs(1))
        .map(|()| Event::Second)
        .map_err(Error::Timer);

    let stdin_stream = spawn_stdin_stream_unbounded()
        .map(|b| Event::Byte(b))
        .map_err(Error::Stdin)
        .chain(once(Ok(Event::Done)));

    let rate = stdin_stream.select(seconds_stream);

    let mut buffer: Vec<u8> = Vec::new();
    let mut n_bytes = 0;
    let mut n_seconds = 0;

    for event in rate.wait() {
        match event {
            Ok(Event::Byte(b)) => {
                buffer.push(b);
                n_bytes += 1
            }
            Ok(Event::Second) => {
                n_seconds += 1;
                println!("{} bytes in {} seconds", n_bytes, n_seconds);
            }
            Ok(Event::Done) => {
                println!("{} bytes in {} seconds", n_bytes, n_seconds);
                println!("done!");

                if let Some(name) = env::args().nth(1) {
                    return out(&buffer, &name);
                } else {
                    return Err("One command line arg (filename) is required.".to_string());
                }
            }
            Err(e) => eprintln!("error {:?}", e),
        }
    }

    Err("Out of loop event.".to_string())
}

fn out(buffer: &[u8], name: &str) -> Result<(), String> {
    println!("  | Writing output ...");
    image::load_from_memory(&buffer)
        .map_err(|err| {
            println!("error while loading image, {}", err.to_string());
            err.to_string()
        })
        .and_then(|image| {
            println!("    | Image loaded, preparing write.");
            let res = image.save(name).map_err(|err| {
                println!("error while writing file.");
                err.to_string()
            });

            println!("    | Completed writing result.");
            res
        })
}
