use backlight::Brightness;

pub struct IntelBacklight {
    device: Brightness,
}

// TODO: Error handling needs to be implemented
impl IntelBacklight {
    pub fn new() -> IntelBacklight {
        IntelBacklight {
            device: Brightness::default(),
        }
    }

    pub fn set_brightness(&self, to: u16) {
        self.device.set_percent(to as i32);
    }

    pub fn get_brightness(&self) -> u16 {
        self.device.get_percent().unwrap() as u16
    }
}