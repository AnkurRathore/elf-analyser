use crate::utils::{read_u16, read_u32, read_u64};
use std::error::Error;

#[derive(Debug)]
pub struct ElfHeader {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
}

pub fn analyze_elf(data: &[u8]) -> Result<(), Box<dyn Error>> {
    // Ensure the file is large enough to contain an ELF header
    if data.len() < 16 {
        return Err("File too small to be a valid ELF file".into());
    }

    // Check the magic number
    let magic = &data[0..4];
    if magic != b"\x7fELF" {
        return Err("Invalid ELF magic number".into());
    }

    // Parse the ELF header
    let class = data[4];
    let data_encoding = data[5];
    let version = data[6];
    let os_abi = data[7];
    let abi_version = data[8];

    let e_type = read_u16(&data[16..18], data_encoding)?;
    let e_machine = read_u16(&data[18..20], data_encoding)?;
    let e_version = read_u32(&data[20..24], data_encoding)?;
    let e_entry = read_u64(&data[24..32], data_encoding)?;

    let header = ElfHeader {
        magic: [magic[0], magic[1], magic[2], magic[3]],
        class,
        data: data_encoding,
        version,
        os_abi,
        abi_version,
        e_type,
        e_machine,
        e_version,
        e_entry,
    };

    println!("ELF Header: {:?}", header);
    Ok(())
}