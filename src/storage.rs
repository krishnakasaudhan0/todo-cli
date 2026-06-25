use std::fs;
use crate::task::Task;
const FILE_PATH : &str = "task.json";

pub fn load_tasks()-> Vec<Task> {
    let data = fs::read_to_string(FILE_PATH);
    match data {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or_else(|_| vec![])
        }
        Err(_) => vec![], // file doesn't exist yet
    }
}
pub fn save_tasks(tasks : &Vec<Task>){
    let data = serde_json::to_string_pretty(tasks)
        .expect("Failed to serialize tasks");

    fs::write(FILE_PATH, data)
        .expect("Failed to write file");

}