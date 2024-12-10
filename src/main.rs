use crossterm::{
    execute,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
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
    enable_raw_mode()?; // Enable raw mode for better input handling
    let mut tasks = load_tasks();
    let mut selected_index = 0;

    loop {
        render_menu(&tasks, selected_index)?;

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
                    if let Some(task) = tasks.get_mut(selected_index) {
                        task.completed = !task.completed;
                    }
                    save_tasks(&tasks);
                }
                KeyCode::Char('a') => {
                    add_task(&mut tasks);
                    save_tasks(&tasks);
                }
                KeyCode::Char('d') => {
                    if !tasks.is_empty() {
                        tasks.remove(selected_index);
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                        save_tasks(&tasks);
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    save_tasks(&tasks);
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?; // Restore terminal mode before exiting
    Ok(())
}

fn render_menu(tasks: &[Task], selected_index: usize) -> std::io::Result<()> {
    // Clear the screen and move the cursor to the top-left corner
    execute!(
        io::stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    println!("Todo CLI - Use ↑/↓ to navigate, SPACE to toggle done, A to add, D to delete, Q to quit\n");

    for (i, task) in tasks.iter().enumerate() {
        // Format the task status as [✓] or [ ]
        let status = if task.completed { "[✓]" } else { "[ ]" };

        // Calculate padding for the task description to align the line
        let padding_width = 1; // Adjust this to set the desired alignment
        let formatted_task = format!("{:<width$} {}", status, task.description, width = padding_width);

        // Move to the correct line and set highlight if selected
        if i == selected_index {
            execute!(io::stdout(), SetBackgroundColor(Color::Blue), SetForegroundColor(Color::Black))?;
        }

        // Print the entire line (status and description)
        print!("{}", formatted_task);

        // Reset color if the task is selected
        if i == selected_index {
            execute!(io::stdout(), ResetColor)?;
        }

        // Print a newline after each task to ensure they appear on separate lines
        println!();
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
