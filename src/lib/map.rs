// map data

// Table 1 - the world, 32x16 room pointers

#[derive(Clone, Copy, Debug, Default)]
pub struct WorldPtr ( [[u8; 32]; 16] );

// Table 2 - rooms, 16 slit pointers

#[derive(Clone, Copy, Debug, Default)]
pub struct RoomPtr ( [u16; 16] );

// Table 3 - a slit of sixteen 2x2 meta-tiles

#[allow(dead_code)]
pub struct SlitPtr ( [u16; 16] );

// Meta-Tile

#[allow(dead_code)]
pub struct MetaTilePtr ( [u8; 4] );

// locations

#[allow(dead_code)]
pub struct MapTable {
    start: usize,
    len: usize,
}

pub const MAP_PTR_TABLES: [[MapTable; 4]; 8] = [
    [ MapTable { start: 0x19F80, len: 0x200 }, MapTable { start: 0x1A180, len:  0xCA0 }, MapTable { start: 0x1AE20, len: 0x36A0 }, MapTable { start:     0x0, len:    0x0 } ], // Big Blue
    [ MapTable { start: 0x1E4E0, len: 0x200 }, MapTable { start: 0x1E6E0, len: 0x1320 }, MapTable { start: 0x1FA00, len: 0x5C00 }, MapTable { start:     0x0, len:    0x0 } ], // Sand Ocean and Silence
    [ MapTable { start: 0x25600, len: 0x200 }, MapTable { start: 0x25800, len:  0xC20 }, MapTable { start: 0x26420, len: 0x3260 }, MapTable { start:     0x0, len:    0x0 } ], // Port Town
    [ MapTable { start: 0x29680, len: 0x200 }, MapTable { start: 0x29880, len:  0x600 }, MapTable { start: 0x29E80, len: 0x2110 }, MapTable { start:     0x0, len:    0x0 } ], // Death Wind
    [ MapTable { start: 0x2BF90, len: 0x200 }, MapTable { start: 0x2C190, len: 0x12E0 }, MapTable { start: 0x2D470, len: 0x2F30 }, MapTable { start:     0x0, len:    0x0 } ], // Red Canyon
    [ MapTable { start: 0x303A0, len: 0x200 }, MapTable { start: 0x305A0, len:  0xE00 }, MapTable { start: 0x313A0, len: 0x5530 }, MapTable { start:     0x0, len:    0x0 } ], // Fire Field
    [ MapTable { start: 0x368D0, len: 0x200 }, MapTable { start: 0x36AD0, len:  0xF80 }, MapTable { start: 0x37A50, len: 0x49B0 }, MapTable { start:     0x0, len:    0x0 } ], // Mute City
    [ MapTable { start: 0x3C400, len: 0x200 }, MapTable { start: 0x3C600, len: 0x1680 }, MapTable { start: 0x3DC80, len: 0x2380 }, MapTable { start: 0x68000, len: 0x50C0 } ], // White Land 1 and 2
];

const MODIFICATION_TABLE: usize = 0x66948;

const MODIFICATION_TABLE_PTR: usize = 0x6693E; // 15 of these

// private helpers

use crate::fetch_byte;
use crate::fetch_word;

fn validate_world(world: usize) -> Result<(), String> {
    if world < 1 || world > 8 {
        return Err(format!("invalid world: {}", world));
    }
    Ok(())
}

// public fn

pub fn get_world_ptr(rom: &[u8], world: usize) -> Result<WorldPtr, String> {
    validate_world(world)?;

    let _maptable: &MapTable = &MAP_PTR_TABLES[world-1][0];

    let mut _worldp: WorldPtr = WorldPtr( [[0xFF; 32]; 16] );
    let mut _loc: usize = 0;
    for row in 0..16 {
        for col in 0..32 {
            _loc = _maptable.start + (row*32) + (col);
            _worldp.0[row][col] = fetch_byte(&rom, _loc)?;
        }
    };

    Ok(_worldp)
}

