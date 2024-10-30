pub mod todo_operations {
    use std::io;

    use postgres::Client;

    use crate::types::Todo;
    pub fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {}
            Err(_no_updates_is_fine) => {}
        }
        input.trim().to_string()
    }

    pub fn get_all_todos(client: &Client) -> Vec<Todo> {
        let todos: Vec<Todo> = Vec::new();
        todos
    }

    pub fn add_todo(title: String, todo_id: u32, client: &Client) {
        let todo = Todo { id, title };
    }

    pub fn mark_todo_as_done(todo_id: String, client: &Client) {
        match todo_id.parse::<u32>() {
            Ok(todo_id) => {
                if let Some(todo) = todos.iter().find(|todo| todo.id == todo_id) {
                    println!("{} is marked as done", todo.title);
                    todos.retain(|todo| todo.id != todo_id);
                } else {
                    println!("Todo with ID {} not found", todo_id);
                }
            }
            Err(_) => {
                println!("Invalid todo ID");
            }
        }
    }
}

pub mod db_connection {
    use dotenv::dotenv;
    use postgres::{Client, NoTls};
    use std::env;
    use std::error::Error;

    pub fn connect_to_db() -> Result<Client, Box<dyn Error>> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DB URL not set");

        let connection = Client::connect(&db_url, NoTls)?;

        Ok(connection)
    }
}
