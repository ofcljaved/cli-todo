use crate::task::Task;

pub struct Todo {
    tasks: Vec<Task>,
}

pub fn display_task_header() {
    println!("| Id  | Status (completed)  | Description  ");
}

impl Todo {
    pub fn new() -> Self {
        Todo { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
        let last_task_added = self.tasks.last().unwrap();
        display_task_header();
        last_task_added.display(self.tasks.len());
    }

    pub fn update_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = !task.completed;
            display_task_header();
            task.display(index + 1);
        } else {
            println!("Inavlid task index");
        }
    }

    pub fn display_task(&mut self) {
        display_task_header();
        for (index, task) in self.tasks.iter().enumerate() {
            task.display(index + 1);
        }
    }
}
