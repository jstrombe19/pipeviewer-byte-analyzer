use clap::{App, Arg};
use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
  let matches = App::new("pipeviewer")
    .arg(Arg::with_name("infile")
      .help("Read from a file instead of stdin")
    )
    .arg(Arg::with_name("outfile")
      .short("o")
      .long("outfile")
      .takes_value(true)
      .help("Write output to a file instead of stdout"),
    )
    .arg(Arg::with_name("silent")
      .short("s")
      .long("silent")
    )
    .get_matches();
    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    let silent = if matches.is_present("silent") {
      true
    } else {
      !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };
    dbg!(infile, outfile, silent);
    // dbg! macro example - NOTE: this is NOT for logging!
    // dbg!(silent);
    let mut buffer = [0; CHUNK_SIZE];
    let mut total_bytes = 0;
    loop {
        // original position of buffer
        // let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        // when using dbg!() if you are also using the += (assignment operator)
        // you will not get a value returned because the assignment operator does
        // not return a value - it returns the unit type or empty type ()
        total_bytes += num_read;
        // simplest solution to check if the value assignment returns an error:
        // io::stdout().write_all(&buffer[..num_read])?;
        // this syntax is intentionally not used in this case to allow the custom
        // case to handle a broken pipe error type
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            // this is the destructured way to access ErrorKind, and requires the explicit
            // inclusion of ErrorKind in the std::io use statement above
            // the other way to do this is to drill down each level:
            // if e.kind() == std::io::ErrorKind::BrokenPipe {}
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            // standard error return
            // return Err(e);
            // can also be customized to handle errors differently
            eprintln!(
                "I'm sorry, Dave. I'm afraid I can't handle this: {}",
                e.to_string()
            );
            // in addition to customized error messages, you can also
            // assign custom standard error codes
            std::process::exit(1);
        }
    }
    if !silent {
        eprintln!("\r{}", total_bytes);
    }
    Ok(())
}
