// car data

// array size for ACCEL_DATA, HANDLE_DATA, SLIDE_DATA
pub const ARRAY_SIZE_19: usize     =    0x13;

// array size for SLIDE_DATA
pub const ARRAY_SIZE_31: usize     =    0x1F;

// array size for BRAKE_DATA, DASH_HANDLE
pub const ARRAY_SIZE_32: usize     =    0x20;

// there are 5 of these, so put them in an array per car
pub const DAMAGE_DATA_SIZE: usize  =     0x5; // [ crash, graze, on wall, out of course, bomb ]

const ACCEL_START: usize           = 0x14A4A; // see note in get_accel_data()
const BRAKE_DATA_START: usize      = 0x14A96;
const DAMAGE_DATA_1_START: usize   =  0x7AB1; // crash
const DAMAGE_DATA_2_START: usize   =  0x7AB5; // graze
const DAMAGE_DATA_3_START: usize   =  0x7AB9; // on wall
const DAMAGE_DATA_4_START: usize   =  0x7ABD; // out of course
const DAMAGE_DATA_5_START: usize   =  0x7AC1; // bomb
const DAMAGE_SPEED_START: usize    =  0x7ACD;
const DAMAGE_TIME_START: usize     =  0x7AED;
const DASH_HANDLE_START: usize     = 0x149F7;
const ENEMY_SPIN_INIT_START: usize =  0x7AE5;
const FRICTION_DATA_START: usize   =  0x7AA9;
const GRIP_LIMIT_START: usize      =  0x7AA1;
const GRIP_VECSPD_START: usize     =  0x7A85;
const HANDLE_DATA_START: usize     = 0x149AB;
const MAXIMUM_SPEED_START: usize   =  0x7A91;
const MYCAR_SPIN_INIT_START: usize =  0x7ADD;
const POWER_DOWN_SENS_START: usize =  0x7AD5;
const REDUCE_DATA_START: usize     =  0x7AAD;
const REPAIR_SPEED_START: usize    =  0x7AC5;
const SLIDE_DATA_START: usize      = 0x14A18;
const SLIP_SPEED_START: usize      =  0x7A99;
const SLIP_VECTOR_START: usize     =  0x7A81;
const SLIP_VECSPD1_START: usize    =  0x7A89;
const SLIP_VECSPD2_START: usize    =  0x7A8D;

#[derive(Clone, Copy, Debug, Default)]
pub struct CarStats {
    accele_data: [u8; ARRAY_SIZE_19],
    brake_data: [u8; ARRAY_SIZE_32], // same for every car
    slip_vector: u8, 
    grip_vecspd: u8, 
    slip_vecspd1: u8, 
    slip_vecspd2: u8, 
    maximum_speed: u16,
    slip_speed: u16,
    grip_limit: u16,
    friction_data: u8, 
    reduce_data: u8, 
    damage_data: [u8; DAMAGE_DATA_SIZE], // [ crash, graze, on wall, out of course, bomb ]
    repair_speed: u16,
    damage_speed: u16,
    power_down_sens: u16,
    mycar_spin_init: u16,
    enemy_spin_init: u16,
    damage_time: u8, // in frames, keep in mind SNES is framelocked to 60FPS (2x interlaced scanlines at NTSC 30FPS)
    handle_data: [u8; ARRAY_SIZE_19],
    dash_handle: [u8; ARRAY_SIZE_32], // same for every car, actually 33 bytes but...
    dash_handle_over: u8, // overflow byte because Rust only impl trait for arrays <=32 members
    slide_data: [u8; ARRAY_SIZE_31], // same for every car
}

// private helpers

use crate::fetch_byte;
use crate::fetch_word;

fn validate_car(car: usize) -> Result<(), String> {
    if car < 1 || car > 4 {
        return Err(format!("invalid car: {}", car));
    }
    Ok(())
}

// public fn

pub fn get_carstats(rom: &[u8], car: usize) -> Result<CarStats, String> {
    validate_car(car)?;

    let mut _carstats = CarStats::default();
    _carstats.accele_data = get_accel_data(&rom, car)?;
    _carstats.brake_data = get_brake_data(&rom)?;
    _carstats.slip_vector = get_slip_vector(&rom, car)?;
    _carstats.grip_vecspd = get_grip_vecspd(&rom, car)?;
    _carstats.slip_vecspd1 = get_slip_vecspd1(&rom, car)?;
    _carstats.slip_vecspd2 = get_slip_vecspd2(&rom, car)?;
    _carstats.maximum_speed = get_maximum_speed(&rom, car)?;
    _carstats.slip_speed = get_slip_speed(&rom, car)?;
    _carstats.grip_limit = get_grip_limit(&rom, car)?;
    _carstats.friction_data = get_friction_data(&rom, car)?;
    _carstats.reduce_data = get_reduce_data(&rom, car)?;
    _carstats.damage_data = get_damage_data(&rom, car)?;
    _carstats.reduce_data = get_reduce_data(&rom, car)?;
    _carstats.damage_data = get_damage_data(&rom, car)?;
    _carstats.repair_speed = get_repair_speed(&rom, car)?;
    _carstats.damage_speed = get_damage_speed(&rom, car)?;
    _carstats.power_down_sens = get_power_down_sens(&rom, car)?;
    _carstats.mycar_spin_init = get_mycar_spin_init(&rom, car)?;
    _carstats.enemy_spin_init = get_enemy_spin_init(&rom, car)?;
    _carstats.damage_time = get_damage_time(&rom, car)?;
    _carstats.handle_data = get_handle_data(&rom, car)?;
    _carstats.dash_handle = get_dash_handle(&rom)?;
    _carstats.dash_handle_over = get_dash_handle_over(&rom)?;
    _carstats.slide_data = get_slide_data(&rom)?;
    Ok(_carstats)
}

// private functions

fn get_accel_data(rom: &[u8], car: usize) -> Result<[u8; ARRAY_SIZE_19], String> {
    validate_car(car)?;
    // there is also data for "car 0" in the source, i assume this is enemy acceleration.
    // it is the same as car 1 so the ACCEL_START position is shifted forward 19 bytes

    let start: usize = ACCEL_START + ((car - 1) * ARRAY_SIZE_19);
    let mut _data: [u8; ARRAY_SIZE_19] = [0; ARRAY_SIZE_19];
    _data.copy_from_slice(&rom[start..start + ARRAY_SIZE_19]);

    Ok(_data)
}

fn get_brake_data(rom: &[u8]) -> Result<[u8; ARRAY_SIZE_32], String> {
    let start: usize = BRAKE_DATA_START;
    let mut _data: [u8; ARRAY_SIZE_32] = [0; ARRAY_SIZE_32];
    _data.copy_from_slice(&rom[start..start + ARRAY_SIZE_32]);

    Ok(_data)
}

fn get_damage_data(rom: &[u8], car: usize) -> Result<[u8; DAMAGE_DATA_SIZE], String> {
    validate_car(car)?;

    let mut _data: [u8; DAMAGE_DATA_SIZE] = [0; DAMAGE_DATA_SIZE];
    _data[0] = fetch_byte(&rom, DAMAGE_DATA_1_START + (car - 1))?;
    _data[1] = fetch_byte(&rom, DAMAGE_DATA_2_START + (car - 1))?;
    _data[2] = fetch_byte(&rom, DAMAGE_DATA_3_START + (car - 1))?;
    _data[3] = fetch_byte(&rom, DAMAGE_DATA_4_START + (car - 1))?;
    _data[4] = fetch_byte(&rom, DAMAGE_DATA_5_START + (car - 1))?;

    Ok(_data)
}

fn get_damage_speed(rom: &[u8], car: usize) -> Result<u16, String> {
    validate_car(car)?;

    let start: usize = DAMAGE_SPEED_START + ((car - 1) * 2);
    let _data: u16 = fetch_word(&rom, start)?;

    Ok(_data)
}

fn get_damage_time(rom: &[u8], car: usize) -> Result<u8, String> {
    validate_car(car)?;

    let _data: u8 = fetch_byte(&rom, DAMAGE_TIME_START + (car - 1))?;

    Ok(_data)
}

fn get_dash_handle(rom: &[u8]) -> Result<[u8; ARRAY_SIZE_32], String> {
    let start: usize = DASH_HANDLE_START;
    let mut _data: [u8; ARRAY_SIZE_32] = [0; ARRAY_SIZE_32];
    _data.copy_from_slice(&rom[start..start + ARRAY_SIZE_32]);

    Ok(_data)
}

fn get_dash_handle_over(rom: &[u8]) -> Result<u8, String> {
    let start: usize = DASH_HANDLE_START + ARRAY_SIZE_32;
    let _data: u8 = fetch_byte(&rom, start)?;

    Ok(_data)
}

fn get_enemy_spin_init(rom: &[u8], car: usize) -> Result<u16, String> {
    validate_car(car)?;

    let start: usize = ENEMY_SPIN_INIT_START + ((car - 1) * 2);
    let _data: u16 = fetch_word(&rom, start)?;

    Ok(_data)
}

fn get_friction_data(rom: &[u8], car: usize) -> Result<u8, String> {
    validate_car(car)?;

    let _data: u8 = fetch_byte(&rom, FRICTION_DATA_START + (car - 1))?;

    Ok(_data)
}

fn get_grip_limit(rom: &[u8], car: usize) -> Result<u16, String> {
    validate_car(car)?;

    let start: usize = GRIP_LIMIT_START + ((car - 1) * 2);
    let _data: u16 = fetch_word(&rom, start)?;

    Ok(_data)
}

fn get_grip_vecspd(rom: &[u8], car: usize) -> Result<u8, String> {
    validate_car(car)?;

    let _data: u8 = fetch_byte(&rom, GRIP_VECSPD_START + (car - 1))?;

    Ok(_data)
}

fn get_handle_data(rom: &[u8], car: usize) -> Result<[u8; ARRAY_SIZE_19], String> {
    validate_car(car)?;

    let start: usize = HANDLE_DATA_START + ((car - 1) * ARRAY_SIZE_19);
    let mut _data: [u8; ARRAY_SIZE_19] = [0; ARRAY_SIZE_19];
    _data.copy_from_slice(&rom[start..start + ARRAY_SIZE_19]);

    Ok(_data)
}

fn get_maximum_speed(rom: &[u8], car: usize) -> Result<u16, String> {
    validate_car(car)?;

    let start: usize = MAXIMUM_SPEED_START + ((car - 1) * 2);
    let _data: u16 = fetch_word(&rom, start)?;

    Ok(_data)
}

fn get_mycar_spin_init(rom: &[u8], car: usize) -> Result<u16, String> {
    validate_car(car)?;

    let start: usize = MYCAR_SPIN_INIT_START + ((car - 1) * 2);
    let _data: u16 = fetch_word(&rom, start)?;

    Ok(_data)
}

fn get_power_down_sens(rom: &[u8], car: usize) -> Result<u16, String> {
    validate_car(car)?;

    let start: usize = POWER_DOWN_SENS_START + ((car - 1) * 2);
    let _data: u16 = fetch_word(&rom, start)?;

    Ok(_data)
}

fn get_reduce_data(rom: &[u8], car: usize) -> Result<u8, String> {
    validate_car(car)?;

    let _data: u8 = fetch_byte(&rom, REDUCE_DATA_START + (car - 1))?;

    Ok(_data)
}

fn get_repair_speed(rom: &[u8], car: usize) -> Result<u16, String> {
    validate_car(car)?;

    let start: usize = REPAIR_SPEED_START + ((car - 1) * 2);
    let _data: u16 = fetch_word(&rom, start)?;

    Ok(_data)
}

fn get_slide_data(rom: &[u8]) -> Result<[u8; ARRAY_SIZE_31], String> {
    let start: usize = SLIDE_DATA_START;
    let mut _data: [u8; ARRAY_SIZE_31] = [0; ARRAY_SIZE_31];
    _data.copy_from_slice(&rom[start..start + ARRAY_SIZE_31]);

    Ok(_data)
}

fn get_slip_speed(rom: &[u8], car: usize) -> Result<u16, String> {
    validate_car(car)?;

    let start: usize = SLIP_SPEED_START + ((car - 1) * 2);
    let _data: u16 = fetch_word(&rom, start)?;

    Ok(_data)
}

fn get_slip_vecspd1(rom: &[u8], car: usize) -> Result<u8, String> {
    validate_car(car)?;

    let _data: u8 = fetch_byte(&rom, SLIP_VECSPD1_START + (car - 1))?;

    Ok(_data)
}

fn get_slip_vecspd2(rom: &[u8], car: usize) -> Result<u8, String> {
    validate_car(car)?;

    let _data: u8 = fetch_byte(&rom, SLIP_VECSPD2_START + (car - 1))?;

    Ok(_data)
}

fn get_slip_vector(rom: &[u8], car: usize) -> Result<u8, String> {
    validate_car(car)?;

    let _data: u8 = fetch_byte(&rom, SLIP_VECTOR_START + (car - 1))?;

    Ok(_data)
}

