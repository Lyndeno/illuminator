use ddc::Ddc;
use ddc_i2c::I2cDeviceDdc;
//use std::env::args;
use chrono::prelude::{DateTime, Local, Datelike};

// Use for smooth brightness
//use std::{thread, time};

static BRIGHT_STEP: u16 = 1;
static VCP_BRIGHTNESS: u8 = 0x10;

static BRIGHTNESS_DAY: u16 = 100;
static BRIGHTNESS_NIGHT: u16 = 30;

// TODO: Split this file into separate files of similar functionality

fn main() {
    //let args = args().nth(1); // take the first arg to be desired brightness

    // parse the brightness to u16
    //let brightness: u16 = args.expect("argument: monitor brightness 0-100").parse::<u16>().ok().expect("This is not an integer!");

    // get monitor device
    // TODO: Get device path from config file or command line
    // TODO: Get device path from model number: eg. "LG QHD"
    let ddc = &mut ddc_i2c::from_i2c_device("/dev/i2c-4").unwrap();

    let local: DateTime<Local> = Local::now();
    let local_unix = local.timestamp();

    // this returns suneset and sunrise as a unix timestamp
    // TODO: Take into account sunrise/sunset of previous and next days IF NEEDED, might not be needed
    let (sunrise_unix, sunset_unix) = sunrise::sunrise_sunset(53.5461, -113.323975, local.year(), local.month(), local.day());

    // check if time is between sunset and sunrise
    if (local_unix < sunset_unix) & (local_unix >= sunrise_unix) {
        set_brightness(ddc, BRIGHTNESS_DAY);
    // check if time is before sunrise or after sunset
    } else if (local_unix < sunrise_unix) | (local_unix >= sunset_unix) {
        set_brightness(ddc, BRIGHTNESS_NIGHT);
    }
}

// this function slowly changes the brightness
// TODO: implement delays from get_step_delay into this function
fn set_brightness(ddc: &mut I2cDeviceDdc, to_val: u16) {
    let mut current_val: u16 = get_brightness(ddc);
    if to_val > current_val { // when increasing brightness
        while to_val > current_val{
            current_val += BRIGHT_STEP;
            ddc.set_vcp_feature(VCP_BRIGHTNESS, current_val).expect("Error emitted");
        }
    } else if to_val < current_val { // when decreasing brightness
        while to_val < current_val{
            current_val -= BRIGHT_STEP;
            ddc.set_vcp_feature(VCP_BRIGHTNESS, current_val).expect("Error emitted");
        }
    } else {
        println!("Nothing happend");
    };
}

// function to get u16 brightness
fn get_brightness(ddc: &mut I2cDeviceDdc) -> u16 {
    // get current brightness info
    let current_val = ddc.get_vcp_feature(VCP_BRIGHTNESS).expect("Failed");
    // return the value of the brightness
    current_val.value()
}

// get amount of time to delay between adjustments of 1% in brightness to get desired transition time
// return value is in milliseconds
fn get_step_delay_ms(delta_brightness: u16, delta_seconds: i64) -> u64 {
    let step_delay_ms: u64 = (delta_seconds as u64 * 1000) / (delta_brightness as u64);
    step_delay_ms
}


