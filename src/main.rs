extern crate byteorder;
extern crate clap;

use std::error::Error as ErrorTrait;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};
use clap::{Arg, App};

const UF2_MAGIC_START0: u32 = 0x0A324655; // "UF2\n"
const UF2_MAGIC_START1: u32 = 0x9E5D5157; // Randomly selected
const UF2_MAGIC_END: u32    = 0x0AB16F30; // Ditto

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("UF2Conv")
        .version("0.1")
        .author("Paul Sajna <sajattack@gmail.com>")
        .about("Converts binary files to Microsoft's UF2 format https://github.com/Microsoft/uf2")
        .arg(Arg::with_name("base")
            .short("b")
            .long("base")
            .help("Sets base address of application for BIN format")
            .default_value("0x2000")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Write output to named file")
            .default_value("flash.uf2"))
        .arg(Arg::with_name("family")
            .short("f")
            .long("family")
            .default_value("0x0")
            .help("specify familyID number")
            .takes_value(true))
        .get_matches();

    let path = Path::new(matches.value_of("INPUT").unwrap());
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let family_id = u32::from_str_radix(
        &matches.value_of("family").unwrap()[2..], 16).unwrap_or(0x0);
    let base = u32::from_str_radix(
        &matches.value_of("base").unwrap()[2..], 16).unwrap_or(0x2000);

    let uf2 = bin_to_uf2(&buffer, family_id, base);

    let outpath = Path::new(matches.value_of("output").unwrap());
    let display = outpath.display();
    let mut outfile = match File::create(&outpath) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(outfile) => outfile,
    };
    outfile.write(&uf2?)?;
    Ok(())
}

fn bin_to_uf2(bytes: &Vec<u8>, family_id: u32, app_start_addr: u32) -> Result<Vec<u8>, std::io::Error> {
    let datapadding = [0u8; 512-256-32-4];
    let nblocks: u32 = ((bytes.len() + 255) / 256) as u32;
    let mut outp: Vec<u8> = Vec::new();
    for blockno in 0..nblocks {
        let ptr = 256 * blockno;
        let mut chunk = match bytes.get(ptr as usize..ptr as usize+256) {
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
        let header: [u32; 8] = [UF2_MAGIC_START0,
            UF2_MAGIC_START1,
            flags,
            ptr + app_start_addr,
            256,
            blockno,
            nblocks,
            family_id];
        let mut block = [0u8; 512];
        for temp in header.iter() {
            (&mut block[..]).write_u32::<LittleEndian>(*temp)?;
        }
        (&mut block[..]).write(&chunk)?;
        (&mut block[..]).write(&datapadding)?;
        (&mut block[..]).write_u32::<LittleEndian>(UF2_MAGIC_END)?;
        outp.write(&block)?;
    }
    Ok(outp)
}
