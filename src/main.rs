use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;
fn main() {
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        io::stdout().write_all(&buffer).unwrap();
    }
    if !silent {
        eprintln!("{}", total_bytes);
    }
}

//env::var
//dbg!

// add git hooks
// vim .git/hooks/pre-commit
//cargo fmt
//exec cargo clipp -- -D warnings
// make it executable: chmod a+x ./git/hooks/pre-commit
