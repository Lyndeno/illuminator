use backlight::Brightness as IntelBrightness;
use crate::brightness::Brightness;

pub struct IntelBacklight {
    device: IntelBrightness,
}

// TODO: Error handling needs to be implemented
impl IntelBacklight {
    pub fn new() -> IntelBacklight {
        IntelBacklight {
            device: IntelBrightness::default(),
        }
    }

}

impl Brightness for IntelBacklight {
    fn set_brightness(&mut self, to: u16) {
        self.device.set_percent(to as i32);
    }

    fn get_brightness(&mut self) -> Option<u16> {
        match self.device.get_percent() {
            Ok(v) => Some(v as u16),
            Err(_) => None,
        }
    }
}