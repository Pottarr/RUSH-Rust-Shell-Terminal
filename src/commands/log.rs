use crate::Terminal;

impl Terminal {
    pub fn log(&mut self, final_output: &mut String) {
        if self.command.len() == 1 {
            final_output.push_str("\nPlease specify the option being logged");
        } else {
            let mut contents = String::new();
            match self.command[1].as_str() {
                "-h" => {
                    contents.push_str("\n");
                    contents.push_str(format!("Command count in history: {}", self.history.len()).as_str());
                    for command in &self.history {
                        contents.push_str("\n");
                        contents.push_str(&command);
                    }
                    if self.command.contains(&">".to_string()) || self.command.contains(&">>".to_string()) {
                        self.redirect_helper(final_output, contents);
                    } else {
                        final_output.push_str(&contents);
                        self.output.push(final_output.to_string());
                    }
                }
                _ => {
                    final_output.push_str("\nInvalid arguments");
                    self.output.push(final_output.to_string());
                }
            }
        }
    }
}