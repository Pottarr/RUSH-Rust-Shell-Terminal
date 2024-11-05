use crate::Terminal;
use std::fs;
use std::path::Path;

impl Terminal {
    pub fn mkdir(&mut self, final_output: &mut String) {
        if self.command.len() == 1 {
            final_output.push_str("Please enter a directory name to create");
        } else {
            let mut dir_count = 0;
            for path in &self.command[1..] {
                if Path::new(path).has_root() {
                    match fs::create_dir(path) {
                        Ok(()) => dir_count += 1,
                        Err(e) => final_output.push_str(format!("\n{}: {}", path, e).as_str()),
                    }
                } else {
                    match fs::create_dir(format!("{}/{}", self.current_path.to_str().unwrap(), path)) {
                        Ok(()) => dir_count += 1,
                        Err(e) => final_output.push_str(format!("\n{}: {}", path, e).as_str()),
                    }
                }
            }
            final_output.push_str(format!("\nCreated {} directory", dir_count).as_str());
        }
        self.output.push(final_output.to_string());
        self.current_command_position = self.history.len()
    }
}