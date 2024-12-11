pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: &str) -> Self {
        Task {
            description: description.to_string(),
            completed: false,
        }
    }

    pub fn display(&self, id: usize) {
        let status = if self.completed { "[X]" } else { "[]" };
        println!("| {:<2}  | {:<18}  | {}", id, status, self.description);
    }
}
