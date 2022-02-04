use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use clap::Parser;
use rand::prelude::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    /// Input file or if not specified, uses stdin.
    #[clap(short = 'f', long)]
    input: Option<String>,

    /// Output file or if not specified, uses stdout.
    #[clap(short = 'o', long)]
    output: Option<String>,

    #[clap(short, long)]
    size: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();

    let stdin = std::io::stdin();
    let reader: Box<dyn BufRead> = match &opts.input {
        Some(filepath) => {
            let file = File::open(filepath)?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(stdin.lock()),
    };

    let stdout = std::io::stdout();
    let writer: Box<dyn Write> = match &opts.output {
        Some(filepath) => {
            let file = File::open(filepath)?;
            Box::new(file)
        }
        None => Box::new(stdout.lock()),
    };

    resevoir_sampler(reader, writer, opts.size)?;

    Ok(())
}

fn resevoir_sampler(
    reader: impl BufRead,
    mut writer: impl Write,
    size: u64,
) -> Result<(), Box<dyn Error>> {
    let size: usize = size.try_into().unwrap();
    let mut resevoir: Vec<String> = Vec::with_capacity(size);

    let mut rng = thread_rng();

    for (i, line) in reader.lines().enumerate() {
        if i < size {
            resevoir.push(line.unwrap());
        } else {
            let r = rng.gen_range(0..size);
            if r < size {
                resevoir[r] = line.unwrap();
            }
        }
    }

    for line in resevoir {
        writeln!(writer, "{}", &line)?;
    }

    Ok(())
}
