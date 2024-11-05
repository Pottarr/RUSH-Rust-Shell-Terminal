use crate::Terminal;
use std::{fs::File, io::Write};
use std::path::Path;

impl Terminal {
    pub fn mkfile(&mut self, final_output: &mut String) {
        if self.command.len() == 1 {
            final_output.push_str("\nPlease enter a file name to create");
        } else {
            let mut file_count= 0;
            let mut file: File;
            for path in &self.command[1..] {
                if Path::new(path).has_root() {
                    file = match File::create_new(path) {
                        Ok(file) => file,
                        Err(e) => {
                            final_output.push_str(format!("\n{}: {}", path, e).as_str());
                            continue
                        }
                    }
                } else {
                    file = match File::create_new(format!("{}/{}", self.current_path.to_str().unwrap(), path)) {
                        Ok(file) => file,
                        Err(e) => {
                            final_output.push_str(format!("\n{}: {}", path, e).as_str());
                            continue
                        }
                    }
                }
                file.write_all("".as_bytes()).unwrap();
                file_count += 1;
            }
            
            final_output.push_str(format!("\nCreated {} file(s)", file_count).as_str());
        }
        self.output.push(final_output.to_string());
        self.current_command_position = self.history.len()
    }
}