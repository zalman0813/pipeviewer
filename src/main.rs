use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Result, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let matches = App::new("pipeviewer")
        .arg(Arg::with_name("infile").help("Read from a file instead of stdin"))
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .help("Write output to a file instead of stdout"),
        )
        .arg(Arg::with_name("silent").short("s").long("silent"))
        .get_matches();
    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    let silent = if matches.is_present("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    let mut buffer = [0; CHUNK_SIZE];
    let mut total_bytes = 0;
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
            // eprint!("Oh no, an error {:?}", buffer);
            // std::process::exit(1);
        }
    }
    Ok(())
}

//env::var
//dbg!

// add git hooks
// vim .git/hooks/pre-commit
//cargo fmt
//exec cargo clipp -- -D warnings
// make it executable: chmod a+x ./git/hooks/pre-commit
//
// Check for a single pattern
// if let Err(e) = ... {
//     expression
// }

// question mark for error result

// Clap

// cargo run -- if you want start the arguments and options for your program
