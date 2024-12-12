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

fn main() {
    let mut todo_app = Todo::new();
    println!("{:=^35}", "");
    println!("{:=^35}", "TODO APP");
    println!("{:=^35}", "");
    loop {
        println!();
        println!("{:-^35}", "");
        println!("| {:<32}|", "Menu Options");
        println!("| {:<32}|", "1. Add new Task");
        println!("| {:<32}|", "2. Toggle Task Completion");
        println!("| {:<32}|", "3. Show Task");
        println!("| {:<32}|", "4. Delete Task");
        println!("| {:<32}|", "5. Exit");
        println!("{:-^35}", "");
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
                todo_app.update_task(index);
            }

            3 => todo_app.display_task(),
            4 => {
                let index: usize = match get_input("Enter Task index to delete it") {
                    Ok(value) => value,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                };
                todo_app.delet_task(index);
            }
            5 => break,
            _ => println!("Invalid option!!!"),
        }
    }
}
