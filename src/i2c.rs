use ddc::Ddc;
use ddc_i2c::I2cDeviceDdc;

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

    pub fn set_brightness(&mut self, to: u16) {
        self.device.set_vcp_feature(VCP_BRIGHTNESS, to);
    }

    pub fn get_brightness(&mut self) -> Result<u16, ddc_i2c::Error<std::io::Error>> {
        match self.device.get_vcp_feature(VCP_BRIGHTNESS) {
            Ok(brightness) => Ok(brightness.value()),
            Err(e) => Err(e),
        }
    }
}