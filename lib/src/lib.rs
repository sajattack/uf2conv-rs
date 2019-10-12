extern crate byteorder;

use std::io::prelude::*;

use byteorder::{LittleEndian, WriteBytesExt};

const UF2_MAGIC_START0: u32 = 0x0A324655; // "UF2\n"
const UF2_MAGIC_START1: u32 = 0x9E5D5157; // Randomly selected
const UF2_MAGIC_END: u32    = 0x0AB16F30; // Ditto

pub fn bin_to_uf2(bytes: &Vec<u8>, family_id: u32, app_start_addr: u32) -> Result<Vec<u8>, std::io::Error> {
    let datapadding = [0u8; 512-256-32-4];
    let nblocks: u32 = ((bytes.len() + 255) / 256) as u32;
    let mut outp: Vec<u8> = Vec::new();
    for blockno in 0..nblocks {
        let ptr = 256 * blockno;
        let chunk = match bytes.get(ptr as usize..ptr as usize+256) {
            Some(bytes) => bytes.to_vec(),
            None => {
                let mut chunk = bytes[ptr as usize..bytes.len()].to_vec();
                while chunk.len() < 256 {
                    chunk.push(0);
                }
                chunk
            }
        };
        let mut flags: u32 = 0;
        if family_id != 0 {
            flags |= 0x2000
        }

        // header
        outp.write_u32::<LittleEndian>(UF2_MAGIC_START0)?;
        outp.write_u32::<LittleEndian>(UF2_MAGIC_START1)?;
        outp.write_u32::<LittleEndian>(flags)?;
        outp.write_u32::<LittleEndian>(ptr + app_start_addr)?;
        outp.write_u32::<LittleEndian>(256)?;
        outp.write_u32::<LittleEndian>(blockno)?;
        outp.write_u32::<LittleEndian>(nblocks)?;
        outp.write_u32::<LittleEndian>(family_id)?;

        // data
        outp.write(&chunk)?;
        outp.write(&datapadding)?;

        // footer
        outp.write_u32::<LittleEndian>(UF2_MAGIC_END)?;
    }
    Ok(outp)
}

