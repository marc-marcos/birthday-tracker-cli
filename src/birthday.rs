#[derive(Debug)]
pub struct Birthday {
    pub name: String,
    pub surname: String,
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Birthday {
    pub fn build(name: &str, surname: &str, day: u8, month: u8, year: u16) -> Birthday {
        Birthday {
            name: String::from(name),
            surname: String::from(surname),
            day,
            month,
            year,
        }
    }
}
