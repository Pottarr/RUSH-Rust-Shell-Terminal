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
                // We have 2 cases for file names
                // First is the name that we supplied has no root
                // for example: hello.txt, this one has no root, it has no slashes infront
                // Second is the name has a root
                // for example /home/useer/Desktop/hello.txt, this one has a root, as seen by the forward slash infront
                // The front slashes may differ from OSes forexample Windows uses C:\\ or a backslash
                if Path::new(path).has_root() {
                    // if the filename has a root, use that as the path
                    file = match File::create_new(path) {
                        Ok(file) => file,
                        Err(e) => {
                            final_output.push_str(format!("\n{}: {}", path, e).as_str());
                            continue
                        }
                    }
                } else {
                    // if no root, add a root into the path to prevent errors
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