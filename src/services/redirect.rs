use crate::Terminal;
use std::{fs::File, io::Write};
use std::path::Path;
use std::fs::OpenOptions;

impl Terminal {
    pub fn redirect_helper(&mut self, final_output: &mut String, contents: String) {
        let index_of_redirect = self.command.iter().position(|x| x == ">" || x == ">>").unwrap();
        match self.command[index_of_redirect..].len() {
            1 => {
                final_output.push('\n');
                final_output.push_str("Error: No filename specified.");
            }
            2 => {
                let stupid_path = self.command[index_of_redirect+1].clone();
                let stupid_path_2 = format!("{}/{}", self.current_path.to_str().unwrap(), self.command[index_of_redirect+1].clone());
                let mut filepath= Path::new(stupid_path.as_str());
                let mut file_open = & mut OpenOptions::new();
                let mut file: File;
                if filepath.has_root() {
                    if filepath.exists() {
                        file_open = file_open.write(true);
                    } else {
                        file_open = file_open.write(true).create_new(true);
                    }
                } else {
                    filepath = Path::new(stupid_path_2.as_str());
                    if filepath.exists() {
                        file_open = file_open.write(true);
                    } else {
                        file_open = file_open.write(true).create_new(true);
                    }
                }
                match self.command[index_of_redirect].clone().as_str() {
                    ">" => {
                        file = file_open.open(filepath).unwrap();
                        if let Err(e) = writeln!(file, "{contents}") {
                            final_output.push('\n');
                            final_output.push_str(format!("Couldn't write to file: {e}").as_str());
                        } else {
                            final_output.push('\n');
                            final_output.push_str(format!("Successfully redirect to: {}", filepath.to_str().unwrap()).as_str());
                        }
                    }
                    ">>" => {
                        file = file_open.append(true).open(filepath).unwrap();
                        if let Err(e) = writeln!(file, "{contents}") {
                            final_output.push('\n');
                            final_output.push_str(format!("Couldn't write to file: {e}").as_str());
                        } else {
                            final_output.push('\n');
                            final_output.push_str(format!("Successfully redirect to: {}", filepath.to_str().unwrap()).as_str());
                        }
                    }
                    _ => unreachable!()
                }
            }
            _ => {
                final_output.push('\n');
                final_output.push_str("Can only redirect to 1 file at a time.");
            }
        }
        self.output.push(final_output.to_string());
    }
}