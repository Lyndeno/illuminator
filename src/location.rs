use chrono::prelude::{Local, Datelike};

pub enum Timeperiod {
    Day,
    Night,
}

pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

impl Timeperiod {
    pub fn get(local: &Location) -> Timeperiod {
        let local_time = Local::now();

        // TODO: Take into account sunrise/sunset of previous and next days IF NEEDED, might not be needed
        let (sunrise_unix, sunset_unix) = sunrise::sunrise_sunset(local.latitude, local.longitude, local_time.year(), local_time.month(), local_time.day());
        let local_timestamp = local_time.timestamp();
        if (local_timestamp < sunset_unix) && (local_timestamp >= sunrise_unix) {
            Timeperiod::Day
        } else {
            Timeperiod::Night
        }
    }
}

impl Location {
    pub fn get_time_period(&self) -> Timeperiod {
        Timeperiod::get(self)
    }
}