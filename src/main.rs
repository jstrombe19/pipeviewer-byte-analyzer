use pipeviewer_byte_analyzer::{args::Args, read, stats, write};
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<()> {
  
  // to view the outcome or values of the variables, use the dbg! macro below
  // dbg!(infile, outfile, silent);
  // dbg! macro example - NOTE: this is NOT for logging!
  // dbg!(silent);

  let args = Args::parse();
  // destructuring of the struct - this is simpler than cloning the struct three times
  let Args {
    infile,
    outfile,
    silent,
  } = args;

  // create quit signals for each child thread and the parent thread. the children
  // are cloned out of the parent
  // the Arc will coordinate cleanup once the threads are no longer in use
  // Mutex functions the same way it does in C
  let quit = Arc::new(Mutex::new(false));
  let (quit1, quit2, quit3) = (quit.clone(), quit.clone(), quit.clone());

  let read_handle = thread::spawn(move || read::read_loop(&infile, quit1));
  let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit3));
  let write_handle = thread::spawn(move || write::write_loop(&outfile, quit3));

  // crash if any threads have crashed
  // `.join()` returns a `thread::Result<io::Result<()>>`
  let read_io_result = read_handle.join().unwrap();
  let stats_io_result = stats_handle.join().unwrap();
  let write_io_result = write_handle.join().unwrap();

  // return an error if any threads returned an error
  read_io_result?;
  stats_io_result?;
  write_io_result?;

  let mut total_bytes = 0;
  loop {
      // original position of buffer
      // let mut buffer = [0; CHUNK_SIZE];

      // when using dbg!() if you are also using the += (assignment operator)
      // you will not get a value returned because the assignment operator does
      // not return a value - it returns the unit type or empty type ()

      // simplest solution to check if the value assignment returns an error:
      // io::stdout().write_all(&buffer[..num_read])?;
      // this syntax is intentionally not used in this case to allow the custom
      // case to handle a broken pipe error type
      let buffer = match read::read(&args.infile) {
        Ok(x) if x.is_empty() => break,
        Ok(x) => x,
        Err(_) => break,
      };
      stats::stats(args.silent, buffer.len(), &mut total_bytes, false);
      if !write::write(&args.outfile, &buffer)? {
        break;
      }
  }
  stats::stats(args.silent, 0, &mut total_bytes, true);
  Ok(())
}
