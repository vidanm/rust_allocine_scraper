use core::fmt::Formatter;
use std::fmt::{Result,Display};

#[derive(Debug)] //allows printing in terminal
pub struct Screening {
    pub version:String,
    pub date_time:String,
    pub location:String,
    pub cinema:String
}

impl Screening {
    pub fn new(version:String,date_time:String,location:String,cinema:String) -> Self{
        Self {version,date_time,location,cinema}
    }
}

impl Default for Screening {
    fn default() -> Self {
        Self {
            version:String::from(""),
            date_time:String::from(""),
            location:String::from(""),
            cinema:String::from("")
        }
    }
}

impl Display for Screening {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {} {} {}", self.version, self.date_time,self.location,self.cinema)
    }
}