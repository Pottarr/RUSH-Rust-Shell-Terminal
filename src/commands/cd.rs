use crate::Terminal;
use std::path::{Path, PathBuf};

impl Terminal {
    // Change the current working directory
    pub fn cd(&mut self, final_output: &mut String) {
        if self.command.len() != 2 {
            final_output.push_str("\nInvalid amount of arguments");
        } else {
            let dir_change_to = match self.command[1].as_str() {
                ".." => {
                    self.current_path.ancestors().nth(1).unwrap().to_path_buf()
                }
                "." => {
                    self.current_path.to_path_buf()
                }
                _ => {
                    if Path::new(self.command[1].as_str()).has_root() {
                        PathBuf::from(self.command[1].as_str())
                    } else {
                        PathBuf::from(format!("{}/{}", self.current_path.to_str().unwrap(), self.command[1].as_str()))
                    }
                }
            };
            if dir_change_to.exists() {
                if dir_change_to.is_file() {
                    final_output.push_str("\nNot a directory");
                } else {
                    self.current_path = dir_change_to.to_path_buf();
                    println!("{:?}", self.current_path);
                    let final_path: String;
                    if cfg!(target_os = "windows") {
                        final_path = self.current_path.to_str().unwrap().replace(r"\\", r"\").replace("/", r"\");
                        println!("{}", &final_path);
                    } else {
                        final_path= self.current_path.to_str().unwrap().to_string();
                        println!("{}", &final_path);

                    }
                    final_output.push_str(format!("\nChange directory to {}", &final_path).as_str());
                }
            } else {
                final_output.push_str("\nNo such file or directory");
            }
            self.output.push(final_output.to_string());
        }
        self.current_command_position = self.history.len()
    }
}