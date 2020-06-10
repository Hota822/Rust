pub enum SortOrder {
    Ascending,
    Descending,
}

// #[derive(Debug, PartialEq)]
#[derive(Debug, )]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.first_name == other.first_name
        && self.last_name == other.last_name
        && self.age == other.age

    }
}

impl Student {
    pub fn new(first_name: &str, last_name: &str, age: u8) -> Self {
        Student {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
        }
    }
}