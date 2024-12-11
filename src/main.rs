use std::{io::{self, Write}, usize};

fn get_numeric_input(prompt: &str) -> Option<u8> {
    println!("{}", prompt);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    match input.trim().parse() {
        Ok(value) => Some(value),
        Err(_) => {
            println!("Inavlid input, enter a number");
            None
        }
    }
}

fn get_string_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    input.trim().to_string()
}

fn main() {
    let mut todo_app = Todo::new();
    println!("--------------------------------------");
    println!("---------------TODO APP---------------");
    println!("--------------------------------------");
    println!();
    loop {
        println!("Menu Options");
        println!("1. Add new Task");
        println!("2. Toggle Task Completion");
        println!("3. Show Task");
        println!("4. Exit");
        let choice = match get_numeric_input("Enter your choice") {
            Some(value) => value,
            None => continue,
        };
        match choice {
            1 => {
                let description = get_string_input("Enter Task description");
                todo_app.add_task(&description);
            }
            2 => {
                let index = match get_numeric_input("Enter Task index to toggle it's completion") {
                    Some(value) => value as usize,
                    None => continue,
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
}

struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    fn new() -> Self {
        Todo { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn update_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = !task.completed;
        }
    }

    fn display_task(&mut self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "[X]" } else { "[]" };
            println!("{} {} : {}", index + 1, task.description, status);
        }
    }
}
