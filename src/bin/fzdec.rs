use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

extern crate argparse;
use argparse::{ArgumentParser, Store, StoreTrue}; // StoreFalse

extern crate crc;
use crc::{crc32};

extern crate fzrom;
use fzrom::car::CarStats;

fn main() {

    let mut filename = String::from("rom.bin");
    let mut action_car: bool = false;
    let mut action_map: bool = false;

    // rom data goes in a u8 array
    //let mut rom: [u8; fzrom::MAX_ROMSIZE] = [0; fzrom::MAX_ROMSIZE];

    // however, don't allocate the entire 512 KiB ROM on the stack
    let mut rom: Box<[u8; fzrom::MAX_ROMSIZE]> = Box::new([0; fzrom::MAX_ROMSIZE]);

    // argument parser block
    {
        let mut ap = ArgumentParser::new();

        ap.set_description("Decode ROM.");

        ap.refer(&mut filename)
            .add_option(&["--file"], Store, "ROM file name");

        ap.refer(&mut action_car)
            .add_option(&["--car"], StoreTrue, "Decode car data");

        ap.refer(&mut action_map)
            .add_option(&["--map"], StoreTrue, "Decode map data");

        ap.parse_args_or_exit();
    }

    // file handling block
    {
        let pathname = Path::new(&filename);

        let mut romfile = match File::open(&pathname) {
            Err(why) => panic!("Could not open file {}: {}", pathname.display(), why),
            Ok(romfile) => romfile,
        };

        match romfile.read_exact(&mut *rom) {
            Err(why) => panic!("Could not read file {}: {}", pathname.display(), why),
            Ok(_) => (),
        }

    }
    // above block scope is over, `romfile` is closed now, but `rom` is still in scope
    // `rom` is a Box, to get to the array inside deref with `*rom`, and to borrow the array use `&*rom`

    // checksum block
    {
        let checksum: u32 = crc32::checksum_ieee(&*rom);
        match checksum {
            // https://superfamicom.org/info/f-zero
            0xAA0E31DE => println!("Original (U) ROM detected."),
            0x7681EFC1 => println!("Original (J) ROM detected."),
            0xF1D8F5DA => println!("Original (E) ROM detected."),
            _          => println!("Unexpected checksum 0x{:08x}. Results may not be as intended!", checksum),
        }
    }

    // car block

    if action_car == true {

        let mut mystats: [CarStats; 4] = [CarStats::default(); 4];

        for car in 0..mystats.len() {
            mystats[car] = match fzrom::car::get_carstats(&*rom, car+1) {
                Err(why) => panic!("could not get carstats: {}", why),
                Ok(stats) => stats,
            };
            println!();
            println!("stats for car {}: {:?}", car+1, mystats[car]);
        }
    }

    // map block

    if action_map == true {

        let mut myworlds: [fzrom::map::WorldPtr; 8] = [fzrom::map::WorldPtr::default(); 8];

        for world in 0..myworlds.len() {

            myworlds[world] = match fzrom::map::get_world_ptr(&*rom, world+1) {
                Err(why)  => panic!("could not get worldptr: {}", why),
                Ok(world) => world,
            };
            println!("world {} pointer table\n{:?}", world+1, myworlds[world]);
        }
    }
}

