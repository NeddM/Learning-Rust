use std::{array, io::stdin};

struct Task {
    name: String,
    description: String,
    done: bool,
}

fn main() {
    menu()
}

fn menu() {
    loop {
        println!(" - - MENU - - ");
        println!("0. Exit");
        println!("1. Create task");
        println!("2. Delete task");
        println!("3. List tasks");

        let mut option = String::new();
        stdin().read_line(&mut option).expect("Not a number");
        let option = option.trim();

        match option {
            "0" => break,
            "1" => {
                println!("1");
                create_task()
            }
            "2" => println!("2"),
            "3" => println!("3"),
            _ => continue,
        }
    }
}

fn create_task() {
    let mut tasks: Vec<Task> = Vec::new();
    // ! Formulario
    println!("Creating a new task...");

    println!("Name: ");
    let mut task_name = String::new();
    stdin().read_line(&mut task_name).expect("Not a string");

    println!("Description: ");
    let mut task_description = String::new();
    stdin()
        .read_line(&mut task_description)
        .expect("Not a string");

    println!("¿Done? y/n: ");
    let task_done: bool;
    let mut task_done_check = String::new();
    stdin()
        .read_line(&mut task_done_check)
        .expect("Not a string");
    let task_done_check = task_done_check.to_uppercase();
    let task_done_check: &str = task_done_check.trim();

    match task_done_check {
        "Y" => task_done = true,
        _ => task_done = false,
    }

    // Agregando valores al objeto
    let new_obj = Task {
        name: task_name,
        description: task_description,
        done: task_done,
    };

    tasks.push(new_obj);
    list_task(tasks)
}

fn list_task(tasks: Vec<Task>) {
    for task in tasks.iter() {
        println!("Task {}: {}, {}", task.name, task.description, task.done);
    }
}
