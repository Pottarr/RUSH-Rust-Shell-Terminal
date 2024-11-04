use crate::Terminal;
use std::io::Read;
use std::{fs::File, io::Write};
use std::path::Path;

impl Terminal {
    pub fn meow(&mut self, final_output: &mut String) {
        if self.command.len() == 1 {
            final_output.push('\n');
            final_output.push_str("Please enter a file name");
            self.output.push(final_output.to_string());
        } else {
            let mut all_contents: Vec<(String, String)> = Vec::new();
            let mut contents = String::new();
            for path in &self.command[1..] {
                if path == &"|" {
                    break
                }
                let mut file: File;
                if Path::new(path).has_root() {
                    file = match File::open(Path::new(path)) {
                        Ok(f) => f,
                        Err(_) => {
                            final_output.push('\n');
                            final_output.push_str(format!("{path}: File doesn't exist").as_str());
                            continue
                        }
                    };
                } else {
                    file = match File::open(Path::new(format!("{}/{path}", self.current_path.to_str().unwrap()).as_str())) {
                        Ok(f) => f,
                        Err(_) => {
                            final_output.push('\n');
                            final_output.push_str(format!("{path}: File doesn't exist").as_str());
                            continue
                        }
                    };
                }
                file.read_to_string(&mut contents);
                all_contents.push((path.to_string(), contents.clone()));
                contents.clear();
            }
            if self.command.contains(&"|".to_string()) {
                let index_of_pipe = self.command.iter().position(|x| x == "|").unwrap();
                match self.command[index_of_pipe..].len() {
                    1 => {
                        final_output.push('\n');
                        final_output.push_str("Error: pipe to nothing");
                        self.output.push(final_output.to_string());
                        } 
                    2 => {
                        match self.command[index_of_pipe+1].clone().as_str() {
                            "find" => {
                                final_output.push('\n');
                                final_output.push_str("Please enter a phrase to find");
                                self.output.push(final_output.to_string());
                            }
                            _ => {
                                final_output.push('\n');
                                final_output.push_str(format!("Can't pipe with command: {}", self.command[index_of_pipe+1].clone()).as_str());
                                self.output.push(final_output.to_string());
                            }
                        }

                    }
                    3 => {
                        let phrase = String::from(self.command[index_of_pipe + 2].clone());
                        match self.command[index_of_pipe+1].clone().as_str() {
                            "find" => {
                                for file in &all_contents {
                                    for line in file.1.split('\n') {
                                        if line.contains(&phrase) {
                                            final_output.push('\n');
                                            final_output.push_str(format!("{}: {}", file.0, line).as_str());
                                        }
                                    }
                                }
                                self.output.push(final_output.to_string());
                            }
                            _ => {
                                final_output.push('\n');
                                final_output.push_str(format!("Can't pipe with command: {}", self.command[index_of_pipe+1].clone()).as_str());
                                self.output.push(final_output.to_string());
                            }
                        }
                    }
                    _ => {
                        match self.command[index_of_pipe+1].clone().as_str() {
                            "find" => {
                                final_output.push('\n');
                                final_output.push_str("Too many number of phrase arguments to find");
                                self.output.push(final_output.to_string());
                            }
                            _ => {
                                final_output.push('\n');
                                final_output.push_str(format!("Can't pipe with command: {}", self.command[index_of_pipe+1].clone()).as_str());
                                self.output.push(final_output.to_string());
                            }
                        }

                    }
                }
            } else {
                for file in &all_contents {
                    final_output.push('\n');
                    final_output.push_str(&file.1);
                }
                self.output.push(final_output.to_string());
            }
        }
    }
}