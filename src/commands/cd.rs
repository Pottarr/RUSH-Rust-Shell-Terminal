use crate::Terminal;
use std::path::{Path, PathBuf};

impl Terminal {
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
                    final_output.push_str(format!("\nChange directory to {:?}", self.current_path.to_str().unwrap()).as_str());
                }
            } else {
                final_output.push_str("\nNo such file or directory");
            }
            self.output.push(final_output.to_string());
        }
    }
}