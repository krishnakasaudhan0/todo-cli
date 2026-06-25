use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple Rust todo CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]

pub enum Command {
    Add {
        title: String,
    },
    List,
    Done {
        id: i32,
    },
    Delete {
        id: i32,
    },
}
// pub fn parse_args(args: Vec<String>) -> Option<Command> {
//     if args.len() < 2 {
//         return None;
//     }

//     match args[1].as_str() {
//         "add" => {
//             if args.len() < 3 {
//                 return None;
//             }
//             Some(Command::Add(args[2..].join(" ")))
//         }

//         "list" => Some(Command::List),

//         "done" => {
//             if args.len() < 3 {
//                 return None;
//             }
//             let id = args[2].parse().ok()?;
//             Some(Command::Done(id))
//         }

//         "delete" => {
//             if args.len() < 3 {
//                 return None;
//             }
//             let id = args[2].parse().ok()?;
//             Some(Command::Delete(id))
//         }

//         _ => None,
//     }
// }
