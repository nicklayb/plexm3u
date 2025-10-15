use std::io::prelude::*;
use std::{fs::File, path::Path};

pub fn write<P: AsRef<Path>>(filename: P, rows: Vec<String>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "#EXTM3U");
    for line in rows {
        writeln!(file, "{}", line);
    }
    Ok(())
}
