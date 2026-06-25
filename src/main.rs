// fn main() {
//     let mut app= TodoApp::new();
//     app.add_task("learn Rust".to_string());
//     app.add_task("build project".to_string());
//     app.display();
// }
// struct Task {
//    id : i32,
//    name : String,

// }
// struct TodoApp{
//     tasks: Vec<Task>,
// }

// impl TodoApp{
//    fn new ()-> Self {
//      TodoApp{
//         tasks : Vec::new(),
//      }
//    }

//    fn add_task(&mut self , name : String){
//        let id=(self.tasks.len() as i32 )+1;
//        let task = Task{
//         id  ,
//         name ,
//        };
//        self.tasks.push(task);
//    }



//     fn display(&self){
//         for i in &self.tasks{
//             println!("{}: {}",i.id,i.name);
//         }
//     }
// }


mod app;
mod task;
mod storage;
mod cli;

use clap::Parser;
use cli::{Cli, Command};
use app::App;

fn main() {
    let cli = Cli::parse();

    let mut app = App::new();

    match cli.command {
        Command::Add { title } => {
            app.add_task(title);
            println!("Task added");
        }

        Command::List => {
            app.list();
        }

        Command::Done { id } => {
            app.mark_done(id);
            println!("Task marked done");
        }

        Command::Delete { id } => {
            app.delete(id);
            println!("Task deleted");
        }
    }
}