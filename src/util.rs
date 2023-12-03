use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read(path: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {
    let file = File::open(path)?;

    Ok(io::BufReader::new(file)
        .lines()
        .collect::<std::io::Result<Vec<String>>>()?)
}
