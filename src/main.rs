use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    description: String,
    completed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
    tasks: HashMap<u32, Task>,
}

fn main() {
    let mut logged_in_user: Option<User> = None;

    loop {
        if logged_in_user.is_none() {
            println!("------------------------------------");
            println!("1. Зареєструватися");
            println!("2. Увійти");
            println!("3. Вийти");

            let choice = get_input("Виберіть опцію: ");
            match choice.trim() {
                "1" => register_user(),
                "2" => logged_in_user = login_user(),
                "3" => break,
                _ => println!("Неправильний ввід. Спробуйте знову."),
            }
        } else {
            println!("------------------------------------");
            println!("1. Додати завдання");
            println!("2. Показати всі завдання");
            println!("3. Редагувати завдання");
            println!("4. Видалити завдання");
            println!("5. Позначити завдання як виконане");
            println!("6. Вийти");

            let choice = get_input("Виберіть опцію: ");
            match choice.trim() {
                "1" => add_task(logged_in_user.as_mut().unwrap()),
                "2" => show_tasks(&logged_in_user.as_ref().unwrap().tasks),
                "3" => edit_task(logged_in_user.as_mut().unwrap()),
                "4" => delete_task(logged_in_user.as_mut().unwrap()),
                "5" => complete_task(logged_in_user.as_mut().unwrap()),
                "6" => {
                    save_user(logged_in_user.as_ref().unwrap());
                    println!("Збережено. Вихід...");
                    break;
                }
                _ => println!("Неправильний ввід. Спробуйте знову."),
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn add_task(user: &mut User) {
    let description = get_input("Введіть опис завдання: ");
    let id = user.tasks.len() as u32 + 1;
    user.tasks.insert(
        id,
        Task {
            description,
            completed: "Ні".to_string(),
        },
    );
    println!("Завдання додано з ID {}", id);
}

fn show_tasks(tasks: &HashMap<u32, Task>) {
    if tasks.is_empty() {
        println!("Завдання ще не були додані.");
        return;
    }
    for (id, task) in tasks {
        println!("ID: {}, Опис: {}, Виконано: {}", id, task.description, task.completed.as_str());
    }
}

fn edit_task(user: &mut User) {
    let id: u32 = get_input("Введіть ID завдання для редагування: ")
        .trim()
        .parse()
        .unwrap_or(0);

    if let Some(task) = user.tasks.get_mut(&id) {
        let new_description = get_input("Введіть новий опис завдання: ");
        task.description = new_description;
        println!("Завдання відредаговано.");
    } else {
        println!("Завдання з таким ID не знайдено.");
    }
}

fn delete_task(user: &mut User) {
    let id: u32 = get_input("Введіть ID завдання для видалення: ")
        .trim()
        .parse()
        .unwrap_or(0);

    if user.tasks.remove(&id).is_some() {
        println!("Завдання видалено.");
    } else {
        println!("Завдання з таким ID не знайдено.");
    }
}

fn complete_task(user: &mut User) {
    let id: u32 = get_input("Введіть ID завдання для позначення як виконане: ")
        .trim()
        .parse()
        .unwrap_or(0);

    if let Some(task) = user.tasks.get_mut(&id) {
        task.completed = "Так".to_string();
        println!("Завдання позначено як виконане.");
    } else {
        println!("Завдання з таким ID не знайдено.");
    }
}

fn save_user(user: &User) {
    let mut users = load_users();

    if let Some(existing_user) = users.iter_mut().find(|u| u.username == user.username) {
        *existing_user = user.clone();
    } else {
        users.push(user.clone());
    }

    save_users(&users);
}

fn load_users() -> Vec<User> {
    if let Ok(data) = fs::read_to_string("users.json") {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_users(users: &[User]) {
    let serialized = serde_json::to_string(users).unwrap();
    fs::write("users.json", serialized).unwrap();
}

fn register_user() {
    let username = get_input("Введіть ім'я користувача: ");
    let password = get_input("Введіть пароль: ");
    let mut users = load_users();

    if users.iter().any(|user| user.username == username) {
        println!("Користувач з таким ім'ям вже існує.");
        return;
    }

    users.push(User {
        username,
        password,
        tasks: HashMap::new(),
    });
    save_users(&users);
    println!("Користувача зареєстровано.");
}

fn login_user() -> Option<User> {
    let username = get_input("Введіть ім'я користувача: ");
    let password = get_input("Введіть пароль: ");
    let users = load_users();

    if let Some(user) = users.iter().find(|user| user.username == username && user.password == password) {
        println!("Успішний вхід.");
        return Some(user.clone());
    }

    println!("Помилка авторизації.");
    None
}
