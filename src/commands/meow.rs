use crate::Terminal;
use std::io::Read;
use std::fs::File;
use std::path::Path;

impl Terminal {
    // concatenate files and print its contents
    // this command can be used with pipes too
    // basically, the command can concatenate any number of files and will mash the contents together
    // into one sigle String variable
    pub fn meow(&mut self, final_output: &mut String) {
        if self.command.len() == 1 {
            final_output.push('\n');
            final_output.push_str("Please enter a file name");
            self.output.push(final_output.to_string());
        } else {
            let mut all_contents: Vec<(String, String)> = Vec::new();
            let mut contents = String::new();
            for path in &self.command[1..] {
                // if a pipe character is found, break the loop
                if path == &"|" {
                    break
                }
                let mut file: File;
                // 2 types of filenames
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
                // use match to handle Result returned from the method read_to_string
                match file.read_to_string(&mut contents) {
                    Ok(_) => println!("File read to string successfully"),
                    Err(e) => println!("Error at File read to string: {}", e)
                }
                all_contents.push((path.to_string(), contents.clone()));
                contents.clear();
            }
            // if the command has a pipe symbol, call the pipe helper function
            if self.command.contains(&"|".to_string()) {
                self.piping_helper(final_output, &all_contents);
            } else {
                for file in &all_contents {
                    final_output.push('\n');
                    final_output.push_str(&file.1);
                }
                self.output.push(final_output.to_string());
            }
        }
        self.current_command_position = self.history.len()
    }
}