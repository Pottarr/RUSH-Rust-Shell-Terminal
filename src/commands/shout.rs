use crate::Terminal;
use std::{fs::File, io::Write};
use std::path::Path;
use std::fs::OpenOptions;

impl Terminal {
    pub fn shout(&mut self, final_output: &mut String) {
        if self.command[1..].contains(&">".to_string()) || self.command[1..].contains(&">>".to_string()) {
            let mut error_yet = false;
            let index = self.command.iter().position(|x| x == ">" || x == ">>").unwrap();
            final_output.push('\n');
            final_output.push_str(self.command[1..index].join(" ").as_str());
            let mut filename: String = String::new();
            match self.command.clone().pop().unwrap().as_str() {
                ">>" | ">" => {
                    final_output.push('\n');
                    final_output.push_str("No filename specified.");
                    self.output.push(final_output.to_string());
                    error_yet = true;
                }
                _ => filename = self.command.clone().pop().unwrap()
            };
            if !error_yet {
                if Path::new(filename.clone().as_str()).has_root() {
                    let filepath = Path::new(filename.as_str());
                    if filepath.exists() {
                        match self.command[index].as_str() {
                            ">" => {
                                let mut file = OpenOptions::new().write(true).open(filepath).unwrap();
                                if let Err(e) = writeln!(file, "{}", self.command[1..index].join(" ")) {
                                    eprintln!("Couldn't write to file: {}", e);
                                }
                            }
                            _ => {
                                let mut file = OpenOptions::new().append(true).open(filepath).unwrap();
                                if let Err(e) = writeln!(file, "{}", self.command[1..index].join(" ")) {
                                    eprintln!("Couldn't write to file: {}", e);
                                }
                            }
                        }
                    } else {
                        let mut file = File::create_new(filepath).unwrap();
                        file.write_all(self.command[1..index].join(" ").as_bytes());
                    }
                } else {
                    let temp = format!("{}/{}", self.current_path.to_str().unwrap(), filename.clone());
                    let filepath = Path::new(temp.as_str());
                    if filepath.exists() {
                        match self.command[index].as_str() {
                            ">" => {
                                let mut file = OpenOptions::new().write(true).open(filepath).unwrap();
                                if let Err(e) = writeln!(file, "{}", self.command[1..index].join(" ")) {
                                    eprintln!("Couldn't write to file: {}", e);
                                }
                            }
                            _ => {
                                let mut file = OpenOptions::new().append(true).open(filepath).unwrap();
                                if let Err(e) = writeln!(file, "{}", self.command[1..index].join(" ")) {
                                    eprintln!("Couldn't write to file: {}", e);
                                }
                            }
                        }
                    } else {
                        let mut file = File::create_new(filepath).unwrap();
                        file.write_all(self.command[1..index].join(" ").as_bytes());
                    }
                }
                println!("Wrote")
            }
        } else {
            final_output.push('\n');
            final_output.push_str(self.command[1..].join(" ").as_str());
            self.output.push(final_output.to_string());
        }
    }
}