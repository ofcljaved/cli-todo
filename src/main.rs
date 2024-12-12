mod task;
mod todo;
use std::io::{self, Write};

use todo::Todo;

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

enum MenuOption {
    AddTask,
    ToggleCompletion,
    ShowTasks,
    DeleteTask,
    Exit,
    ClearScreen,
}

impl MenuOption {
    fn from_u8(input: u8) -> Option<Self> {
        match input {
            1 => Some(Self::AddTask),
            2 => Some(Self::ToggleCompletion),
            3 => Some(Self::ShowTasks),
            4 => Some(Self::DeleteTask),
            5 => Some(Self::Exit),
            6 => Some(Self::ClearScreen),
            _ => None,
        }
    }
    fn display_option() {
        println!();
        println!("{:-^35}", "");
        println!("| {:<32}|", "Menu Options");
        println!("| {:<32}|", "1. Add new Task");
        println!("| {:<32}|", "2. Toggle Task Completion");
        println!("| {:<32}|", "3. Show Task");
        println!("| {:<32}|", "4. Delete Task");
        println!("| {:<32}|", "5. Exit");
        println!("| {:<32}|", "6. Clear Terminal");
        println!("{:-^35}", "");
    }
}

fn get_valid_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        match get_input::<T>(prompt) {
            Ok(value) => return value,
            Err(err) => println!("{}", err),
        }
    }
}

fn main() {
    let mut todo_app = Todo::new();
    print!("\x1Bc");
    println!("{:=^35}", "");
    println!("{:=^35}", "TODO APP");
    println!("{:=^35}", "");
    loop {
        MenuOption::display_option();
        let choice: u8 = get_valid_input("Enter your choice");
        match MenuOption::from_u8(choice) {
            Some(MenuOption::AddTask) => {
                let description: String = get_valid_input("Enter Task description");
                todo_app.add_task(description);
            }
            Some(MenuOption::ToggleCompletion) => {
                let index: usize = get_valid_input("Enter Task index to toggle it's completion");
                todo_app.update_task(index);
            }

            Some(MenuOption::ShowTasks) => todo_app.display_task(),
            Some(MenuOption::DeleteTask) => {
                let index: usize = get_valid_input("Enter Task index to delete it");
                todo_app.delet_task(index);
            }
            Some(MenuOption::Exit) => break,
            Some(MenuOption::ClearScreen) => print!("\x1Bc"),
            None => println!("Invalid option!!!"),
        }
    }
}
