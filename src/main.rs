use ncurses::*;
use std::cmp::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn begin_list(&mut self, id: Id, prefix: &str) {
        assert!(self.list_curr.is_none(), "Nested list are not allowed!");
        self.list_curr = Some(id);
    }

    fn list_element(&mut self, label: &str, id: Id) {
        let id_curr = self.list_curr.expect("Not allowed to create list elements outside of list");

        self.label(&format!(label), {
            if todo_curr == index {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        });
    }

    fn label(&mut self, text: &str, pait: i16){
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row +=1;
    }

    fn end_list(&mut self) {
        self.list_curr = None;
    }
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let todos: Vec<String> = vec![
        "write".to_string(),
        "sleep".to_string(),
        "coffee".to_string()
    ];
    let mut todo_curr: usize = 0;
    let dones: Vec<String> = vec![
        "car".to_string(),
        "man".to_string()
    ];
    let mut done_current: usize = 0;

    let mut ui = Ui::default();
    while !quit {
 //       clear(); // Clear the screen before redrawing
        ui.begin(0, 0);
        {
            ui.label("TODO:", REGULAR_PAIR);
            ui.begin_list(todo_curr);
            for (index, todo) in todos.iter().enumerate() {
                ui.list_element(&format!("- [ ] {}", todo), index);
            }
            ui.end_list();

            ui.label("------------------------------", REGULAR_PAIR);

            ui.label("DONE:", REGULAR_PAIR);
            ui.begin_list();
            for (index, done) in dones.iter().enumerate() {
                ui.list_element(&format!("- [x] {}", done), index + 6969);
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
