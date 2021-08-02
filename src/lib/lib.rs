// a library

pub mod car;
pub mod map;

pub const MAX_ROMSIZE: usize       = 0x80000;

// "fetch" - blind data copy

fn fetch_byte(rom: &[u8], start: usize) -> Result<u8, String> {
    if start > MAX_ROMSIZE {
        return Err(String::from("byte fetch out of range"));
    }
    Ok(rom[start])
}

fn fetch_word(rom: &[u8], start: usize) -> Result<u16, String> {
    let mut _data: u16 = 0;
    // TODO: use a proper LE/BE library here?
    _data = (fetch_byte(&rom, start+0x01)? as u16) << 0x8 | (fetch_byte(&rom, start+0x00)? as u16);
    Ok(_data)
}

