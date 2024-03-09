// pub use screening::Screening;
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