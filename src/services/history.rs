use crate::Terminal;
use home::home_dir;
use iced::{keyboard, Event};
use std::{
    fs::{create_dir_all, exists, File, OpenOptions},
    io::{BufWriter, Write},
    process,
};

enum SearchDirection {
    Up,
    Down,
}

impl Terminal {

    pub fn startup_sync_history(&mut self) {
        let config_dir_str: String = format!("{}/.rush", home_dir().unwrap().to_str().unwrap());
        let history_file_str: String = format!("{}/.rush/rush.log", home_dir().unwrap().to_str().unwrap());

        match exists(&config_dir_str) {
            Ok(okay) => match okay {
                true => {
                    println!("Found Config Dir");
                    match exists(&history_file_str) {
                        Ok(okay) => match okay {
                            true => {
                                println!("Log File Found");
                            }
                            false => {
                                println!("Log File not found");
                                match File::create(&history_file_str) {
                                    Ok(_) => {
                                        println!("Log File CREATED SUCCESSFULLY");
                                    }
                                    Err(e) => {
                                        println!("Error at StaSynHis01: {}", e);
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            println!("Error at StaSynHis02: {}", e);
                        }
                    }
                }
                false => {
                    println!("Config Dir not found");
                    match create_dir_all(&config_dir_str) {
                        Ok(_) => {
                            println!("Config Dir and Log File CREATED SUCCESSFULLY");
                            match File::create(&history_file_str) {
                                Ok(_) => {
                                    println!("Log File CREATED SUCCESSFULLY");
                                }
                                Err(e) => {
                                    println!("Error at StaSynHis03: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error at StaSynHis04: {}", e);
                        }
                    }
                }
            },
            Err(e) => {
                println!("Error at StaSynHis05: {}", e);
                process::exit(2);
            }
        }
    }

    pub fn history(&mut self, event: Event) {
        if let Event::Keyboard(keyboard_event) = event {
            match keyboard_event {
                keyboard::Event::KeyPressed { key, .. } => match key {
                    keyboard::Key::Named(named_key) => match named_key {
                        keyboard::key::Named::ArrowUp => self.search_history(SearchDirection::Up),
                        keyboard::key::Named::ArrowDown => {
                            self.search_history(SearchDirection::Down)
                        }
                        _ => self.sync_history(),
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
        let history_file_str: String =
            format!("{}/.rush/rush.log", home_dir().unwrap().to_str().unwrap());

        match exists(&config_dir_str) {
            Ok(okay) => match okay {
                true => {
                    println!("Found Config Dir");
                    match exists(&history_file_str) {
                        Ok(okay) => match okay {
                            true => {
                                println!("Log File Found");
                                match OpenOptions::new().write(true).truncate(true).open(&history_file_str) {
                                    Ok(file) => {
                                        let mut writer = BufWriter::new(file);
                                        for latest_command in &self.history {
                                            match writeln!(writer, "{}", latest_command) {
                                                Ok(_) => {}
                                                Err(e) => {
                                                    println!("Error at SynHis01: {}", e);
                                                    process::exit(2);
                                                }
                                            }
                                        }
                                        println!("Commands SYNCED");
                                    }
                                    Err(e) => {
                                        println!("Error 011: {}", e);
                                        // process::exit(2);
                                    }
                                }
                            }
                            false => {
                                println!("Log File not found");
                                match File::create(&history_file_str) {
                                    Ok(_) => println!("Log File CREATED SUCCESSFULLY"),
                                    Err(e) => {
                                        println!("Error at 012: {}", e);
                                        process::exit(2);
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            println!("Error at 013: {}", e);
                            process::exit(2);
                        }
                    }
                }
                false => {
                    println!("Config Dir not found");
                    match create_dir_all(&config_dir_str) {
                        Ok(_) => {
                            println!("Config Dir and Log File CREATED SUCCESSFULLY");
                            match File::create(&history_file_str) {
                                Ok(_) => println!("Log File CREATED SUCCESSFULLY"),
                                Err(e) => {
                                    println!("Errorat 014: {}", e);
                                    process::exit(2);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error at 015: {}", e);
                            process::exit(2);
                        }
                    }
                }
            },
            Err(e) => {
                println!("Error at 016: {}", e);
                process::exit(2);
            }
        }
    }

    pub fn push_history(&mut self, content: &String) {
        println!("Logged");
        self.history.push(content.clone());
    }

    // fn update_history(&self) {
    //     todo!()
    // }

    fn search_history(&mut self, direction: SearchDirection) {
        self.sync_history();

        // let history_len = &self.history.len();

        match direction {
            SearchDirection::Up => {

                println!("Search Up");
            }
            SearchDirection::Down => {
                println!("Search Down");
            }
        }
    }
}