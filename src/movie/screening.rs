// pub use screening::Screening;
#[derive(Debug)] //allows printing in terminal
pub struct Screening {
    pub date_time:String,
    pub location:String,
    pub cinema:String
}

impl Screening {
    pub fn new(date_time:String,location:String,cinema:String) -> Self{
        Self {date_time,location,cinema}
    }
}

impl Default for Screening {
    fn default() -> Self {
        Self {
            date_time:String::from(""),
            location:String::from(""),
            cinema:String::from("")
        }
    }
}