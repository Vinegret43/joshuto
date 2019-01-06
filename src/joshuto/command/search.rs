extern crate ncurses;

use std;
use std::fmt;

use joshuto;
use joshuto::input;
use joshuto::window;

use joshuto::command;

#[derive(Debug)]
pub struct Search;

impl Search {
    pub fn new() -> Self { Search }
    pub fn command() -> &'static str { "search" }
}

impl command::JoshutoCommand for Search {}

impl std::fmt::Display for Search {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", Self::command())
    }
}

impl command::Runnable for Search {
    fn execute(&self, context: &mut joshuto::JoshutoContext)
    {
        let mut term_rows: i32 = 0;
        let mut term_cols: i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);

        let win = window::JoshutoPanel::new(1, term_cols, (term_rows as usize - 1, 0));
        ncurses::keypad(win.win, true);

        const PROMPT: &str = ":search ";
        ncurses::waddstr(win.win, PROMPT);

        win.move_to_top();
        ncurses::doupdate();

        let mut index: Option<i32> = None;

        if let Some(user_input) = input::get_str(&win, (0, PROMPT.len() as i32)) {
            if let Some(curr_list) = context.curr_list.as_ref() {
                for (i, dirent) in (&curr_list.contents).into_iter().enumerate() {
                    if dirent.file_name_as_string.contains(user_input.as_str()) {
                        index = Some(i as i32);
                        break;
                    }
                }
            }
        }

        if let Some(index) = index {
            command::CursorMove::cursor_move(index, context);
        }

        win.destroy();
        ncurses::update_panels();
        ncurses::doupdate();
    }
}