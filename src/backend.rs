use crate::i2c::I2cBacklight;
use crate::intel::IntelBacklight;

pub enum Backend {
    I2c(I2cBacklight),
    Intel(IntelBacklight),
}

impl Backend {
    pub fn set_brightness(&mut self, to: u16) {
        match self {
            Backend::I2c(i) => i.set_brightness(to),
            Backend::Intel(i) => i.set_brightness(to),
        };
    }
}