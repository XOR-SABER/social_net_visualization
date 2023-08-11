
use core::fmt;
pub struct Friend {
    name: String,
}

//Associated Function
impl Friend {
    pub fn new(field1: String) -> Self {
        return Friend { name: field1 };
    }
}

//Cool way todo operator overloading
impl fmt::Display for Friend {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(format, "Name: {}", self.name);
    }
}