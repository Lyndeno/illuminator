use ddc::Ddc;
use ddc_i2c::I2cDeviceDdc;
use std::env::args;

static BRIGHT_STEP: u16 = 1;

fn main() {
    let args = args().nth(1); // take the first arg to be desired brightness

    // parse the brightness to u16
    let brightness: u16 = args.expect("argument: monitor brightness 0-100").parse::<u16>().ok().expect("This is not an integer!");

    // get monitor device
    let ddc = &mut ddc_i2c::from_i2c_device("/dev/i2c-4").unwrap();

    let (sunrise, sunset) = sunrise::sunrise_sunset(53.5461, -113.323975, 2020, 12, 19); // thiis returns julian dates

    let current_brightness = get_brightness(ddc);

    set_brightness(ddc, brightness, current_brightness);
}

// this function slowly changes the brightness
fn set_brightness(ddc: &mut I2cDeviceDdc, to_val: u16, mut current_val: u16) {
    if to_val > current_val { // when increasing brightness
        while to_val > current_val{
            current_val += BRIGHT_STEP;
            ddc.set_vcp_feature(0x10, current_val).expect("Error emitted");
        }
    } else if to_val < current_val { // when decreasing brightness
        while to_val < current_val{
            current_val -= BRIGHT_STEP;
            ddc.set_vcp_feature(0x10, current_val).expect("Error emitted");
        }
    } else {
        println!("Nothing happend");
    };
}

// function to get u16 brightness
fn get_brightness(ddc: &mut I2cDeviceDdc) -> u16 {
    // get current brightness info
    let current_val = ddc.get_vcp_feature(0x10).expect("Failed");
    // return the value of the brightness
    current_val.value()
}
