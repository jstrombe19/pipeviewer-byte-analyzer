use pipeviewer_byte_analyzer::{args::Args, read, stats, write};
use std::io::Result;

fn main() -> Result<()> {
  
  // to view the outcome or values of the variables, use the dbg! macro below
  // dbg!(infile, outfile, silent);
  // dbg! macro example - NOTE: this is NOT for logging!
  // dbg!(silent);

  let args = Args::parse();
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
