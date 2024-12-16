use ncurses::*;
use std::cmp::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
}

impl Ui {
    fn begin(&mut self) {
        // TODO:
    }

    fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Nested list are not allowed!");
        self.list_curr = Some(id);
    }

    fn list_element(&mut self, label: &str, id: Id) {
            // let pair = {
            //     if todo_curr == index {
            //         HIGHLIGHT_PAIR
            //     } else {
            //         REGULAR_PAIR
            //     }
            // };
            //
            // attron(COLOR_PAIR(pair));
            // mv(index as i32, 1);
            // let _ = addstr(todo);
            // attroff(COLOR_PAIR(pair));
        // TODO:
    }

    fn label(&mut self, text: &str){}

    fn end_list(&mut self) {
        // TODO:
    }
    
    fn end_list() {
        // TODO:
    }
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBILE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let todos: Vec<String> = vec![
        "write".to_string, "sleep".to_string, "coffee".to_string];
    let mut todo_curr: usize = 1;
    let dones = Vec::<String>::new();
    let mut done_current: usize = 0;

    let mut ui = Ui::default();
    while !quit {
 //       clear(); // Clear the screen before redrawing
        ui.begin();
        {
            ui.begin_list(todo_curr);
            for (index, todo) in todos.iter().enumerate() {
                ui.list_element(todo, index);
            }
            ui.end_list();

            ui.label("------------------------------");

            ui.begin_list();
            for (index, done) in dones.iter().enumerate() {
                ui.list_element(done, index);
            }
            ui.end_list();
        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'j' => {
                if todo_curr < todos.len() - 1 {
                    todo_curr += 1;
                }
            }
            'k' => todo_curr = min(todo_curr + 1, todos.len() -1),
            _ => {}
        }
    }

    endwin();
}
