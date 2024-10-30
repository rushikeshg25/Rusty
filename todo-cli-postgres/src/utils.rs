pub mod todo_operations {
    use std::io;

    use postgres::{Client, Row};

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

    pub fn get_all_todos(client: &mut Client) -> Vec<Todo> {
        let mut todos = Vec::new();

        let rows = client
            .query("SELECT todo_id, title FROM person", &[])
            .expect("Query failed");

        for row in rows {
            let todo = row_to_todo(row);
            todos.push(todo);
        }

        todos
    }

    fn row_to_todo(row: Row) -> Todo {
        Todo {
            todo_id: row.get("todo_id"),
            title: row.get("title"),
        }
    }

    pub fn add_todo(title: String, todo_id: u32, client: &mut Client) {
        match client.execute(
            "INSERT INTO todos (todo_id, title) VALUES ($1, $2)",
            &[&todo_id, &title],
        ) {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    pub fn mark_todo_as_done(todo_id: String, client: &mut Client) {
        match client.execute("DELETE FROM todos WHERE todo_id = $1", &[&todo_id]) {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
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
        println!("DB URL: {}", db_url);
        let connection = Client::connect(&db_url, NoTls)?;

        Ok(connection)
    }
}
