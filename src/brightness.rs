pub trait Brightness {
    fn set_brightness(&mut self, to: u16) -> Result< (), () >;

    fn get_brightness(&mut self) -> Option<u16>;
}