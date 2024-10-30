pub mod utils {
    use std::io;

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

    pub fn add_todo(todos: &mut Vec<Todo>, id: u32, title: String) {
        let todo = Todo { id, title };
        todos.push(todo);
    }

    pub fn mark_todo_as_done(todos: &mut Vec<Todo>, todo_id: String) {
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
