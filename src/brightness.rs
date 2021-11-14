pub trait Brightness {
    fn set_brightness(&mut self, to: u16);

    fn get_brightness(&mut self) -> Option<u16>;
}