//use std::env::args;
use chrono::prelude::{DateTime, Local, Datelike};

mod i2c;
use crate::i2c::I2cBacklight;

// Use for smooth brightness
use std::time;

use structopt::StructOpt;

enum Timeperiod {
    Day,
    Night,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "illuminator")]
struct Opt {

    /// Day brightness as a percentage
    #[structopt(long, default_value = "100")]
    brightness_day: u16,

    /// Night brightness as a percentage
    #[structopt(long, default_value = "50")]
    brightness_night: u16,

    /// Step size of brightness as a percentage
    #[structopt(long, default_value = "1")]
    brightness_step: u16,

    /// Transition duration between night and day
    #[structopt(long, default_value = "0")]
    transition_dur: u16,

    /// Path to i2c display
    #[structopt(long)]
    i2c: Vec<String>,
}

// TODO: Split this file into separate files of similar functionality

fn main() {
    let opt = Opt::from_args();

    // get monitor device
    // TODO: Get device path from model number: eg. "LG QHD"
    let mut backlights: Vec<I2cBacklight> = Vec::new();
    //let mut backlight = I2cBacklight::new(opt.display).unwrap();
    for i in 0..opt.i2c.len() {
        backlights.push(I2cBacklight::new(opt.i2c[i].clone()).unwrap());
    }

    loop {
        let local: DateTime<Local> = Local::now();
        let local_unix = local.timestamp();

        // this returns suneset and sunrise as a unix timestamp
        // TODO: Take into account sunrise/sunset of previous and next days IF NEEDED, might not be needed
        let (sunrise_unix, sunset_unix) = sunrise::sunrise_sunset(53.5461, -113.323975, local.year(), local.month(), local.day());
        
        /*
        let current_brightness = match backlight.get_brightness() {
            Ok(value) => value,
            Err(_) => continue,
        };*/

        match get_time_period(local_unix, sunset_unix, sunrise_unix) {
            Timeperiod::Day => {
                //if current_brightness != bright_day {
                    for i in 0..opt.i2c.len() {
                        backlights[i].set_brightness(opt.brightness_day);
                    }
                    println!("Day");
                //}
            }, 
            Timeperiod::Night => {
                //if current_brightness != bright_night {
                    for i in 0..opt.i2c.len() {
                        backlights[i].set_brightness(opt.brightness_night);
                    }
                    println!("Night");
                //}
            },
        };
    }
}

/*
// this function slowly changes the brightness
fn set_brightness(ddc: &mut I2cDeviceDdc, to_val: u16, duration_s: i64, bright_step: u16, smooth: bool) {
    let current_val = get_brightness(ddc);
    
    let mut current_val = match current_val {
        Ok(value) => value,
        Err(_) => return,
    };
    if current_val != to_val {
        // use this value to store the next brightness value
        let mut next_val = current_val;
        let step_delay = get_step_delay( ( (to_val as i32) - (current_val as i32)).abs() as u16, duration_s, bright_step );
        
        while current_val != to_val {
            if smooth {
                thread::sleep(step_delay);
                // set the next brightness value depending on current state
                if (((to_val as i32) - (current_val as i32)).abs() as u16) < bright_step {
                    // reduce step size so we don't infinitely hover around target brightness
                    next_val = to_val;
                } else if current_val < to_val {
                    next_val = current_val + bright_step;
                } else if current_val > to_val {
                    next_val = current_val - bright_step;
                }
            }
            match ddc.set_vcp_feature(VCP_BRIGHTNESS, next_val) {
                Ok(_) => {
                    current_val = next_val; // if operation was valid then current brightness can be stored
                    println!("Transitioning ({}%)", current_val);
                },
                Err(_) => println!("Error writing to monitor device"), //if operation not complete then do nothing and re-loop
            };
        }
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
*/

// get amount of time to delay between adjustments of 1% in brightness to get desired transition time
// return value is the duration type
fn get_step_delay(delta_brightness: u16, delta_seconds: i64, bright_step: u16) -> time::Duration {
    let step_delay_ms: u64 = (delta_seconds as u64 * 1000) / ( (delta_brightness / bright_step) as u64);
    time::Duration::from_millis(step_delay_ms)
}

fn get_time_period(current: i64, sunset: i64, sunrise: i64) -> Timeperiod {
    let mut current_period = Timeperiod::Day;
    if (current < sunset) & (current >= sunrise) {
        current_period = Timeperiod::Day;        
    // check if time is before sunrise or after sunset
    } else if (current < sunrise) | (current >= sunset) {
        current_period = Timeperiod::Night;           
    }
    current_period
}


