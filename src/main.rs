use std::io::{self, Write};

fn get_input<T: std::str::FromStr>(prompt: &str) -> Result<T, String> {
    print!("=> {} : ", prompt);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Input Error: Failed to read line")?;
    input
        .trim()
        .parse::<T>()
        .map_err(|_| "Input Error: Invalid Input".to_string())
}

fn main() {
    let mut todo_app = Todo::new();
    println!("======================================");
    println!("===============TODO APP===============");
    println!("======================================");
    println!();
    loop {
        println!("-----------------------------");
        println!("| Menu Options              |");
        println!("| 1. Add new Task           |");
        println!("| 2. Toggle Task Completion |");
        println!("| 3. Show Task              |");
        println!("| 4. Exit                   |");
        println!("-----------------------------");
        let choice: u8 = match get_input("Enter your choice") {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        match choice {
            1 => {
                let description: String = match get_input("Enter Task description") {
                    Ok(desc) => desc,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };
                todo_app.add_task(&description);
            }
            2 => {
                let index: usize = match get_input("Enter Task index to toggle it's completion") {
                    Ok(value) => value,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };
                todo_app.update_task(index - 1);
            }

            3 => todo_app.display_task(),
            4 => break,
            _ => println!("Invalid option!!!"),
        }
    }
}
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: &str) -> Self {
        Task {
            description: description.to_string(),
            completed: false,
        }
    }

    fn display(&self, id: usize) {
        let status = if self.completed { "[X]" } else { "[]" };
        println!("| {:<2}  | {:<18}  | {}", id, status, self.description);
    }
}

struct Todo {
    tasks: Vec<Task>,
}

fn display_task_header() {
    println!("| Id  | Status (completed)  | Description  ");
}

impl Todo {
    fn new() -> Self {
        Todo { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
        let last_task_added = self.tasks.last().unwrap();
        display_task_header();
        last_task_added.display(self.tasks.len());
    }

    fn update_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = !task.completed;
            display_task_header();
            task.display(index + 1);
        } else {
            println!("Inavlid task index");
        }
    }

    fn display_task(&mut self) {
        display_task_header();
        for (index, task) in self.tasks.iter().enumerate() {
            task.display(index + 1);
        }
    }
}
