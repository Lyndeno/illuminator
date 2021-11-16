mod i2c;
use crate::i2c::I2cBacklight;

mod intel;
use crate::intel::IntelBacklight;

mod brightness;
use crate::brightness::Brightness;

mod location;
use crate::location::{Location, Timeperiod};

// Use for smooth brightness
use std::time;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "illuminator")]
struct Opt {
    /// Verboseness (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

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

    #[structopt(long)]
    intel: bool,
}

// TODO: Split this file into separate files of similar functionality

fn main() {
    let opt = Opt::from_args();

    // Initialize backlight vector
    let mut backlights: Vec<Box<dyn Brightness>> = Vec::new();

    // Add intel backlights to vector
    if opt.intel {
        backlights.push(Box::new(IntelBacklight::new()));
    }

    // Add i2c backlights to vector
    let i2c_count = opt.i2c.len();
    for i in 0..i2c_count {
        backlights.push(Box::new(I2cBacklight::new(opt.i2c[i].clone()).unwrap()));
    }

    loop {
        let local = Location {
            latitude: 53.5461,
            longitude: -113.323975
        };
        
        for bl in &mut backlights {
            bl.set_brightness(match local.get_time_period() {
                Timeperiod::Day => opt.brightness_day,
                Timeperiod::Night => opt.brightness_night,
            });
        }
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