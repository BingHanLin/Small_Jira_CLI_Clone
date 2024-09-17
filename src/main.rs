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
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_string()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        if let Some(page) = navigator.get_current_page() {
            if let Err(e) = page.draw_page() {
                println!("Error in drawing page: {}\nPress any key to continue...", e);

                wait_for_key_press();
            }

            let user_input = get_user_input();

            match page.handle_input(user_input.trim()) {
                Err(e) => {
                    println!(
                        "Error in handling input: {}\nPress any key to continue...",
                        e
                    );

                    wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(e) = navigator.handle_action(action) {
                            print!(
                                "Error in handling action: {}\nPress any key to continue...",
                                e
                            );
                            wait_for_key_press();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}
