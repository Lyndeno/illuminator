use ddc::Ddc;
use ddc_i2c::I2cDeviceDdc;
//use std::env::args;
use chrono::prelude::{DateTime, Local, Datelike};

#[macro_use]
extern crate clap;
use clap::App;

// Use for smooth brightness
use std::{thread, time};

//static BRIGHT_STEP: u16 = 1;
static VCP_BRIGHTNESS: u8 = 0x10;

//static BRIGHTNESS_DAY: u16 = 100;
//static BRIGHTNESS_NIGHT: u16 = 50;

// TODO: Split this file into separate files of similar functionality

fn main() {
    //let args = args().nth(1); // take the first arg to be desired brightness

    let yam1 = load_yaml!("cli.yml");
    let cli_args = App::from_yaml(yam1).get_matches();

    let display_path = cli_args.value_of("display_path").unwrap();
    let transition_dur: i64 = cli_args.value_of("transition_dur_s").unwrap_or("0").parse::<i64>().unwrap();
    let bright_day = cli_args.value_of("brightness_day").unwrap_or("100").parse::<u16>().unwrap();
    let bright_night = cli_args.value_of("brightness_night").unwrap_or("50").parse::<u16>().unwrap();
    let bright_step = cli_args.value_of("brightness_step").unwrap_or("1").parse::<u16>().unwrap();

    // parse the brightness to u16
    //let brightness: u16 = args.expect("argument: monitor brightness 0-100").parse::<u16>().ok().expect("This is not an integer!");

    // get monitor device
    // TODO: Get device path from model number: eg. "LG QHD"
    let ddc = &mut ddc_i2c::from_i2c_device(display_path.to_string()).unwrap();

    loop {
        let local: DateTime<Local> = Local::now();
        let local_unix = local.timestamp();

        // this returns suneset and sunrise as a unix timestamp
        // TODO: Take into account sunrise/sunset of previous and next days IF NEEDED, might not be needed
        let (sunrise_unix, sunset_unix) = sunrise::sunrise_sunset(53.5461, -113.323975, local.year(), local.month(), local.day());
        
        let current_brightness = match get_brightness(ddc) {
            Ok(value) => value,
            Err(_) => continue,
        };

        // check if time is between sunset and sunrise
        if (local_unix < sunset_unix) & (local_unix >= sunrise_unix) {
            if current_brightness != bright_day {
                set_brightness(ddc, bright_day, transition_dur, bright_step);
                println!("Day");
            };
            
        // check if time is before sunrise or after sunset
        } else if (local_unix < sunrise_unix) | (local_unix >= sunset_unix) {
            if current_brightness != bright_night {
                set_brightness(ddc, bright_night, transition_dur, bright_step);
                println!("Night");
            };            
        }
    }
}

// this function slowly changes the brightness
fn set_brightness(ddc: &mut I2cDeviceDdc, to_val: u16, duration_s: i64, bright_step: u16) {
    let current_val = get_brightness(ddc);
    let mut current_val = match current_val {
        Ok(value) => value,
        Err(_) => return,
    };

    // use this value to store the next brightness value
    let mut next_val = current_val;
    let step_delay = get_step_delay( ( (to_val as i32) - (current_val as i32)).abs() as u16, duration_s, bright_step );
    
    while current_val != to_val {
        // set the next brightness value depending on current state
        if (((to_val as i32) - (current_val as i32)).abs() as u16) < bright_step {
            // reduce step size so we don't infinitely hover around target brightness
            next_val = to_val;
        } else if current_val < to_val {
            next_val = current_val + bright_step;
        } else if current_val > to_val {
            next_val = current_val - bright_step;
        }
        match ddc.set_vcp_feature(VCP_BRIGHTNESS, next_val) {
            Ok(_) => {
                current_val = next_val; // if operation was valid then current brightness can be stored
                println!("Transitioning ({}%)", current_val);
            },
            Err(_) => println!("Error writing to monitor device"), //if operation not complete then do nothing and re-loop
        };
        thread::sleep(step_delay);
    }
}

// function to get u16 brightness
fn get_brightness(ddc: &mut I2cDeviceDdc) -> Result<u16, ddc_i2c::Error<std::io::Error>> {
    // get current brightness info
    match ddc.get_vcp_feature(VCP_BRIGHTNESS) {
        Ok(brightness) => Ok(brightness.value()),
        Err(error) => Err(error),
    }
}

// get amount of time to delay between adjustments of 1% in brightness to get desired transition time
// return value is in milliseconds
fn get_step_delay(delta_brightness: u16, delta_seconds: i64, bright_step: u16) -> time::Duration {
    let step_delay_ms: u64 = (delta_seconds as u64 * 1000) / ( (delta_brightness / bright_step) as u64);
    time::Duration::from_millis(step_delay_ms)
}


