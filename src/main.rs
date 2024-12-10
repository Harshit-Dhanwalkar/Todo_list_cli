use crossterm::{
    execute,
    style::{Color, ResetColor, SetBackgroundColor},
    event::{self, Event, KeyCode},
    terminal::{self, ClearType, enable_raw_mode, disable_raw_mode},
    cursor,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

const FILE_PATH: &str = "tasks.json";

fn main() -> std::io::Result<()> {
    enable_raw_mode()?; // Enable raw mode
    let mut tasks = load_tasks();
    let mut selected_index = 0;

    loop {
        // Render the menu
        render_menu(&tasks, selected_index)?;

        // Wait for user input
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_index < tasks.len().saturating_sub(1) {
                        selected_index += 1;
                    }
                }
                KeyCode::Char(' ') => {
                    // Toggle completion status
                    if let Some(task) = tasks.get_mut(selected_index) {
                        task.completed = !task.completed;
                    }
                    save_tasks(&tasks);
                }
                KeyCode::Char('a') => {
                    // Add a new task
                    add_task(&mut tasks);
                    save_tasks(&tasks);
                }
                KeyCode::Char('d') => {
                    // Delete the selected task
                    if !tasks.is_empty() {
                        tasks.remove(selected_index);
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                        save_tasks(&tasks);
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    // Exit the program
                    save_tasks(&tasks);
                    println!("Goodbye!");
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?; // Disable raw mode
    Ok(())
}

fn render_menu(tasks: &[Task], selected_index: usize) -> std::io::Result<()> {
    execute!(
        io::stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    println!("Todo CLI - Use ↑/↓ to navigate, SPACE to toggle done, A to add, D to delete, Q to quit\n");

    for (i, task) in tasks.iter().enumerate() {
        if i == selected_index {
            execute!(io::stdout(), SetBackgroundColor(Color::Blue))?;
        }

        let status = if task.completed { "[✓]" } else { "[ ]" };
        println!("{} {}", status, task.description);

        if i == selected_index {
            execute!(io::stdout(), ResetColor)?;
        }
    }

    Ok(())
}

fn add_task(tasks: &mut Vec<Task>) {
    disable_raw_mode().unwrap(); // Temporarily disable raw mode for user input
    print!("Enter task description: ");
    io::stdout().flush().unwrap();

    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();
    let description = description.trim().to_string();

    let id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    tasks.push(Task {
        id,
        description,
        completed: false,
    });

    enable_raw_mode().unwrap(); // Re-enable raw mode
}

fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks.");
    fs::write(FILE_PATH, json).expect("Failed to save tasks.");
}

fn load_tasks() -> Vec<Task> {
    if let Ok(contents) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}
