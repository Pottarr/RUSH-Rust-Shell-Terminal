use crate::Terminal;
use std::io::Read;
use std::fs::File;
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
                match file.read_to_string(&mut contents) {
                    Ok(_) => println!("File read to string successfully"),
                    Err(e) => println!("Error at File read to string: {}", e)
                }
                all_contents.push((path.to_string(), contents.clone()));
                contents.clear();
            }
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
    }
}