#[derive(Debug)]
pub struct ToDo {
    pub id: u8,
    pub description: String,
    pub is_complete: bool,
}

impl ToDo {
    pub fn new(description: String) -> Self {
        ToDo {
            id: 1,
            description,
            is_complete: false,
        }
    }

    pub fn complete(&mut self) {
        self.is_complete = true
    }

    pub fn display(&self) {
        println!("ID: {}", self.id);
        println!("ToDo: {}", self.description);
        println!("Completed: {}", if self.is_complete { "Yes" } else { "No" });
    }
}