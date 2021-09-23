use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    // dbg! macro example - NOTE: this is NOT for logging!
    // dbg!(silent);
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        // when using dbg!() if you are also using the += (assignment operator)
        // you will not get a value returned because the assignment operator does
        // not return a value - it returns the unit type or empty type ()
        total_bytes += num_read;
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }
    if !silent {
        eprintln!("{}", total_bytes);
    }
}
