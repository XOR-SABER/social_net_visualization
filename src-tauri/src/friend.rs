pub struct Friend {
    name: String,
}

//Associated Function
impl Friend {
    pub fn new(field1: String) -> Self {
        return Friend { name: field1 };
    }
}