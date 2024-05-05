use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let data = "./data/db.json".to_owned();
    let mut nav = Navigator::new(Rc::new(JiraDatabase::new(data)));
    loop {
        clearscreen::clear().unwrap();
        let Some(page) = nav.get_current_page() else {
            break;
        };

        let Ok(_) = page.draw_page() else {
            println!("Something went wrong. Press any key to try again.");
            wait_for_key_press();
            continue;
        };

        let input = get_user_input().trim().to_owned();
        let Ok(Some(action)) = page.handle_input(input.as_str()) else {
            println!("{input} is not an option. Press any key to try again.");
            wait_for_key_press();
            continue;
        };
        if let Err(e) = nav.handle_action(action) {
            println!("Something went wrong: {e}. Press any key to try again.");
            wait_for_key_press();
            continue;
        };
    }
}
