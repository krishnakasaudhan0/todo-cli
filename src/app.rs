use crate::storage::{load_tasks, save_tasks};
use crate::task::Task;
pub struct App {
    pub tasks: Vec<Task>,
}
impl App {
    pub fn new() -> Self {
        let tasks = load_tasks();

        Self { tasks }
    }
    pub fn add_task(&mut self, title: String) {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        self.tasks.push(Task {
            id,
            title,
            done: false,
        });

        save_tasks(&self.tasks);
    }
    pub fn list(&self) {
        for t in &self.tasks {
            let status = if t.done { "✔" } else { "✘" };
            println!("{} [{}] {}", t.id, status, t.title);
        }
    }
    pub fn mark_done(&mut self,id:i32){
        for task in &mut self.tasks{
            if task.id== id {
                task.done=true;

            }
        }
        save_tasks(&self.tasks);

    }
    pub fn delete(&mut self, id: i32) {
    self.tasks.retain(|t| t.id != id);
    save_tasks(&self.tasks);
}
}
