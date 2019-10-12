extern crate byteorder;
extern crate clap;
extern crate uf2;

use std::error::Error as ErrorTrait;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::{Arg, App};
use uf2::bin_to_uf2;

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

