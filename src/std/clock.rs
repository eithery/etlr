use chrono::{DateTime, Local};


pub(crate) trait Clock {
    fn now() -> DateTime<Local>;
}


pub(super) struct SystemClock;


impl Clock for SystemClock {
    fn now() -> DateTime<Local> {
        Local::now()
    }
}
