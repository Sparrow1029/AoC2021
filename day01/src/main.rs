use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};


fn read_ints<R: Read>(io: R) -> Result<Vec<usize>, Error> {
    let br = BufReader::new(io);
    let mut values = vec![];
    for line in br.lines() {
        values.push(line?
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(values)
}

fn main() -> Result<(), Error> {
    // let input_path = PathBuf::from("day01.txt");
    let puzzle_input = read_ints(File::open("day01.txt")?)?;
    Ok(())
}
