use std::error::Error;

pub fn read_u16(data: &[u8], encoding: u8) -> Result<u16, Box<dyn Error>> {
    match encoding{
        1 => Ok(u16::from_le_bytes(data.try_into()?)), // Little endian
        2 => Ok(u16::from_be_bytes(data.try_into()?)), // Big endian
        _ => Err("Invalid encoding".into())
    }
}

pub fn read_u32(data: &[u8], encoding: u8) -> Result<u32, Box<dyn Error>> {
    match encoding{
        1 => Ok(u32::from_le_bytes(data.try_into()?)), // Little endian
        2 => Ok(u32::from_be_bytes(data.try_into()?)), // Big endian
        _ => Err("Invalid encoding".into())
    }
}

pub fn read_u64(data: &[u8], encoding: u8) -> Result<u64, Box<dyn Error>> {
    match encoding{
        1 => Ok(u64::from_le_bytes(data.try_into()?)), // Little endian
        2 => Ok(u64::from_be_bytes(data.try_into()?)), // Big endian
        _ => Err("Invalid encoding".into())
    }
}
