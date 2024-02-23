use crate::error::crab_error;
use std::{
    fs,
    io::{Read, Write},
};

pub fn read_as_bytes(filepath: &'static str) -> Result<Vec<u8>, crab_error::Error> {
    let mut file = fs::File::open(filepath)?;

    let mut buffer = Vec::<u8>::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn write_bytes(filepath: &'static str, data: &Vec<u8>) -> Result<(), crab_error::Error> {
    let mut file = fs::File::create(filepath)?;
    file.write_all(&data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_file() -> Result<(), crab_error::Error> {
        let data = b"Radha Krsna".to_vec();

        write_bytes("./radha.krsna", &data).unwrap();

        let read_data = read_as_bytes("./radha.krsna");
        assert!(read_data.is_ok());
        let read_data = read_data.unwrap();

        assert_eq!(data, read_data);
        Ok(())
    }
}
