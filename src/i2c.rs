use ddc::Ddc;
use ddc_i2c::I2cDeviceDdc;
use std::io::{Error, ErrorKind};

use crate::brightness::Brightness;

static VCP_BRIGHTNESS: u8 = 0x10;

pub struct I2cBacklight {
    device: I2cDeviceDdc,
}

impl I2cBacklight {
    pub fn new(i2c_path: String) -> Result<I2cBacklight, std::io::Error> {
        match ddc_i2c::from_i2c_device(i2c_path) {
            Ok(backlight) => Ok(I2cBacklight {
                device: backlight,
            }),
            Err(e) => Err(e),
        }
    }

    
}

impl Brightness for I2cBacklight {

    fn set_brightness(&mut self, to: u16) -> Result<(), Error > {
        match self.device.set_vcp_feature(VCP_BRIGHTNESS, to) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    fn get_brightness(&mut self) -> Option<u16> {
        match self.device.get_vcp_feature(VCP_BRIGHTNESS) {
            Ok(brightness) => Some(brightness.value()),
            Err(_) => None,
        }
    }
}