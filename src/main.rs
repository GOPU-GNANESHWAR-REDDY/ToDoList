use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
}

const FILE_NAME: &str = "tasks.json";

fn load_tasks() -> VecDeque<Task> {
    let mut file = match File::open(FILE_NAME) {
        Ok(file) => file,
        Err(_) => return VecDeque::new(),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();
    
    serde_json::from_str(&contents).unwrap_or_else(|_| VecDeque::new())
}

fn save_tasks(tasks: &VecDeque<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(FILE_NAME).expect("Failed to open file");
    file.write_all(json.as_bytes()).expect("Failed to write tasks");
}

fn main() {
    let mut tasks = load_tasks();
    
    loop {
        println!("\n--- ToDoList CLI ---");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Remove Task");
        println!("4. Exit");
        println!("Choose an option: ");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter task description:");
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).expect("Failed to read input");

                let new_task = Task {
                    id: tasks.len() + 1,
                    description: desc.trim().to_string(),
                };
                tasks.push_back(new_task);
                save_tasks(&tasks);
                println!("Task added!");
            }
            "2" => {
                if tasks.is_empty() {
                    println!("No tasks available.");
                } else {
                    for task in &tasks {
                        println!("[{}] {}", task.id, task.description);
                    }
                }
            }
            "3" => {
                println!("Enter task ID to remove:");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read input");

                if let Ok(id) = id_input.trim().parse::<usize>() {
                    tasks.retain(|task| task.id != id);
                    save_tasks(&tasks);
                    println!("Task removed.");
                } else {
                    println!("Invalid ID!");
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}

