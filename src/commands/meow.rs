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
            let mut contents = String::new();
            for path in &self.command[1..] {
                if Path::new(path).has_root() {
                    let mut file = match File::open(Path::new(path)) {
                        Ok(f) => f,
                        Err(_) => {
                            final_output.push('\n');
                            final_output.push_str(format!("{path}: File doesn't exist").as_str());
                            continue
                        }
                    };
                    file.read_to_string(&mut contents);
                } else {
                    let mut file = match File::open(Path::new(format!("{}/{path}", self.current_path.to_str().unwrap()).as_str())) {
                        Ok(f) => f,
                        Err(_) => {
                            final_output.push('\n');
                            final_output.push_str(format!("{path}: File doesn't exist").as_str());
                            continue
                        }
                    };
                    file.read_to_string(&mut contents);
                }
            }
            final_output.push('\n');
            final_output.push_str(&contents);
            self.output.push(final_output.to_string());
        }
    }
}