use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    initscr();

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK); // Added semicolon
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE); // Added semicolon

    let mut quit = false;
    let todos = vec!["write", "sleep", "coffee"];
    let mut todo_curr: usize = 0; // Fixed variable name and type

    while !quit {
 //       clear(); // Clear the screen before redrawing

        for (row, todo) in todos.iter().enumerate() {
            let pair = if todo_curr == row {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            };

            attron(COLOR_PAIR(pair));
            mv(row as i32, 0);
            let _ = addstr(todo);
            attroff(COLOR_PAIR(pair));
        }

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'j' => {
                if todo_curr < todos.len() - 1 {
                    todo_curr += 1;
                }
            }
            'k' => todo_curr = min(todo_curr + 1, todo.len()),
            _ => {}
        }
    }

    endwin();
}
