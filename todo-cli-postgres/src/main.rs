mod types;
mod utils;

use crate::utils::db_connection::connect_to_db;
use utils::todo_operations::{add_todo, get_all_todos, get_input, mark_todo_as_done};

fn main() {
    let mut client = match connect_to_db() {
        Ok(client) => client,
        Err(e) => panic!("Error connecting to DB: {}", e),
    };
    let mut generated_id: u32 = 0;
    loop {
        println!("");
        println!("Select an option:");
        println!("1. Add a todo");
        println!("2. List all todos");
        println!("3. Mark a todo as done and delete it");
        println!("4. Ctrl+C to Exit");
        println!("");

        let user_input = get_input("Enter your choice: ");

        match user_input.as_str() {
            "1" => {
                let todo_title = get_input("Enter the todo title: ");
                generated_id += 1;
                add_todo(todo_title, generated_id, &mut client);
            }
            "2" => {
                println!("Listing all todos:");
                get_all_todos(&mut client);
            }
            "3" => {
                println!("Enter Todo ID to mark as done and delete it:");
                let todo_id = get_input("Enter the todo id: ");
                mark_todo_as_done(todo_id, &mut client);
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid input. Please choose a valid option (1-4)");
            }
        }
    }
}
