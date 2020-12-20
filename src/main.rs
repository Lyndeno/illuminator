use ddc::Ddc;
use ddc_i2c::I2cDeviceDdc;
use std::env::args;

static BRIGHT_STEP: u16 = 1;

fn main() {
    //println!("Hello, world!");

    let args = args().nth(1);

    let mut brightness: u16 = args.expect("argument: monitor brightness 0-100").parse::<u16>().ok().expect("This is not an integer!");
    let ddc = &mut ddc_i2c::from_i2c_device("/dev/i2c-4").unwrap();

    let test = get_brightness(ddc);

    set_brightness(ddc, brightness, 50);
}

fn set_brightness(ddc: &mut I2cDeviceDdc, mut to_val: u16, mut current_val: u16) {
    let brightness_increment: u16 = 1;
    if to_val > current_val {
        while to_val > current_val{
            current_val += BRIGHT_STEP;
            ddc.set_vcp_feature(0x10, current_val).expect("Error emitted");
        }
    } else if to_val < current_val {
        while to_val < current_val{
            current_val -= BRIGHT_STEP;
            ddc.set_vcp_feature(0x10, current_val).expect("Error emitted");
        }
    } else {
        println!("Nothing happend");
    };
}

fn get_brightness(ddc: &mut I2cDeviceDdc) -> u16 {
    let current_val = ddc.get_vcp_feature(0x10).expect("Failed");
    //println!("{:?}", current_val.value());
    current_val.value()
}
