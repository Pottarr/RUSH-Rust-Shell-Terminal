use std::{ fs::{create_dir_all, exists}, process};

use crate::Terminal;
use home::home_dir;
use std::fs::File;
use iced::{keyboard, Event};

enum SearchDirection {
    Up,
    Down
}

impl Terminal {
    pub fn history(&mut self, event: Event) {

        if let Event::Keyboard(keyboard_event) = event {
            match keyboard_event {
                keyboard::Event::KeyPressed { key, .. } => match key {
                    keyboard::Key::Named(named_key) => match named_key {
                        keyboard::key::Named::ArrowUp => self.search_history(SearchDirection::Up),
                        keyboard::key::Named::ArrowDown => self.search_history(SearchDirection::Down),
                        _ => println!("some other named key, history updated"),
                    },
                    keyboard::Key::Character(_) => println!("character"),
                    keyboard::Key::Unidentified => println!("unidentified"),
                },
                _ => {}
            }
        }
    }

    fn sync_history(&mut self) {
        
        let config_dir_str: String = format!("{}/.rush", home_dir().unwrap().to_str().unwrap());
        let history_file_str: String = format!("{}/.rush/rush.log", home_dir().unwrap().to_str().unwrap());

        match exists(&config_dir_str) {
            Ok(okay) => {
                match okay {
                    true => {
                        println!("Found Config Dir");
                        match exists(&history_file_str) {
                            Ok(okay) => {
                                match okay {
                                    true => {
                                        println!("Log File Found");
                                    }
                                    false => {
                                        println!("Log File not found");
                                        process::exit(2);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                process::exit(2);
                            }
                        }
                    }
                    false => {
                        println!("Config Dir not found");
                        match create_dir_all(&config_dir_str) {
                            Ok(_) => {
                                println!("Config Dir CREARTED SUCCESSFULLY");
                                match File::create(&history_file_str) {
                                    Ok(_) => println!("Log File CREARTED SUCCESSFULLY"),
                                    Err(e) => {
                                        println!("Error: {}", e);
                                        process::exit(2);
                                    }
                                }

                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                process::exit(2);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                process::exit(2);
            }
        }


    }

    // fn update_history(&self) {
    //     todo!()
    // }

    fn search_history(&mut self, direction: SearchDirection) {
        self.sync_history();

        match direction {
            SearchDirection::Up => {
                println!("Up");
            }
            SearchDirection::Down => {
                println!("Down");
            }
        }

    }
}
