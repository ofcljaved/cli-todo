#[derive(Debug)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

pub fn display_task_header(commnad_executed: &str) {
    println!("{:-^35}", commnad_executed);
    println!("|{:>3} | {:^8} | Description", "Id", "Status");
}

impl Task {
    pub fn new(description: &str) -> Self {
        Task {
            description: description.to_string(),
            completed: false,
        }
    }

    pub fn display(&self, id: usize) {
        let status = if self.completed { "[X]" } else { "[ ]" };
        println!("|{:>3} | {:^8} | {}", id, status, self.description);
    }
}
