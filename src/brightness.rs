pub trait Brightness {
    fn set_brightness(&mut self, to: u16) -> Result< (), std::io::Error >;

    fn get_brightness(&mut self) -> Option<u16>;
}