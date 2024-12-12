use crate::task::{display_task_header, Task};

pub struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    pub fn new() -> Self {
        Todo { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
        let last_task_added = self.tasks.last().unwrap();
        display_task_header("Task Added");
        last_task_added.display(self.tasks.len());
    }

    pub fn update_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index - 1) {
            task.completed = !task.completed;
            display_task_header("Task Updated");
            task.display(index);
        } else {
            println!("Inavlid index: {}", index);
        }
    }

    pub fn display_task(&mut self) {
        display_task_header("Current Tasks");
        for (index, task) in self.tasks.iter().enumerate() {
            task.display(index + 1);
        }
    }

    pub fn delet_task(&mut self, index: usize) {
        if self.tasks.get_mut(index - 1).is_some() {
            let task = self.tasks.remove(index - 1);
            display_task_header("Task Deleted");
            task.display(index);
        } else {
            println!("Inavlid index: {}", index);
        }
    }
}
