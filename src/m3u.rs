use std::io::{self, prelude::*};
use std::{fs::File, path::Path};

const HEADER_LINE: &str = "#EXTM3U";

#[derive(Debug, Clone)]
pub struct Item {
    pub path: String,
}

impl Item {
    pub fn new(path: String) -> Item {
        Item { path }
    }

    pub fn exists_at(&self, root_path: &Path) -> bool {
        root_path.join(self.path.clone()).exists()
    }
}

pub fn write<P: AsRef<Path>>(filename: P, rows: Vec<Item>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "{}", HEADER_LINE)?;
    for line in rows {
        writeln!(file, "{}", line.path)?;
    }
    Ok(())
}

pub fn read<P: AsRef<Path>>(filename: P) -> std::io::Result<Vec<Item>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let mut output_lines: Vec<Item> = Vec::new();

    if let Some(Ok(first_line)) = lines.next()
        && first_line == HEADER_LINE
    {
        for line_result in lines {
            let item = Item::new(line_result?);
            output_lines.push(item);
        }

        Ok(output_lines)
    } else {
        panic!("M3u playlists must start with #EXTM3U")
    }
}
