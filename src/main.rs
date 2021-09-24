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
  let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit2));
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

  Ok(())
}
