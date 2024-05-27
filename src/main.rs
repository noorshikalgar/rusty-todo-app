use std::io::{self, Write};
mod todo;

use termion::clear;
use todo::ToDo;

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("Welcome to Awesome Todo.");
    println!("- - - - - - - - - - - - - - - - - - - - - - - - ");

    let mut todos: Vec<ToDo> = Vec::new();

    loop {
        // Clear the screen
        println!("\n");
        println!("\n");
        println!("\n");
        println!("(l) List Todos");
        println!("(a) Add Todo");
        println!("(c) Mark Complete");
        println!("(q) Quit");
        println!("\n");
        println!("\n");

        let choice = get_input("Enter your choice: ");
        print!("{}", clear::All);
        io::stdout().flush().expect("Failed to flush stdout");
        match choice.as_str() {
            "l" => {
                if todos.iter().count() == 0 {
                    println!("Nothing to show!.")
                }
                if todos.iter().filter(|todo| !todo.is_complete).count() > 0 {
                    println!("In-complete Todos:");
                    println!("--");
                    for todo in todos.iter().filter(|todo| !todo.is_complete) {
                        todo.display();
                        println!("--");
                    }
                }

                if todos.iter().filter(|todo| todo.is_complete).count() > 0 {
                    println!("--------------------------");
                    println!("Completed todos: ");
                    println!("--");

                    for todo in todos.iter().filter(|todo| todo.is_complete) {
                        todo.display();
                        println!("--");
                    }
                }
            }
            "c" => {
                println!("\nEnter ID of the ToDo: ",);
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let id = input.trim().parse::<u8>().unwrap();
                if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
                    todo.complete()
                };
                println!("Todo completed. Hurayy!");
            }
            "a" => {
                println!("Enter description of todo: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut new_todo = ToDo::new(input.trim().to_string());
                new_todo.id = (todos.len() + 1) as u8;
                todos.push(new_todo);
                println!("ToDo added.");
                println!("Hint!. press 'l' to list all todos.");
            }
            "q" => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Wrong key, try again...");
                println!("- - - - - - - - - - - - - - - - - - - - - - - - ");
                println!("Keep this in mind to use the tool.");
                println!("press 'l' to list all todos.");
                println!("press 'a' to add todo.");
                println!("press 'q' to quit. \n");
            }
        }
    }
}
