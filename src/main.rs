use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::io::{self, stdout, BufReader, Read};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Config {
    #[structopt()]
    input: OsString,
}

fn main() -> io::Result<()> {
    let Config { input } = Config::from_args();

    let input = File::open(input)?;
    let input = BufReader::new(input);

    let mut distr = [0usize; u8::MAX as usize];

    for b in input.bytes() {
        let b = b.expect("read byte");

        distr[b as usize] += 1;
    }

    let out = stdout();
    let mut out = out.lock();

    writeln!(&mut out, "byte,frequency")?;
    for (index, frequency) in distr.into_iter().enumerate() {
        writeln!(&mut out, "{index},{frequency}")?;
    }

    out.flush()?;

    Ok(())
}
