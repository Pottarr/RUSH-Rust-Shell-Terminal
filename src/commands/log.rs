use crate::Terminal;

impl Terminal {
    pub fn log(&mut self, final_output: &mut String) {
        if self.command.len() == 1 {
            final_output.push_str("Please specify the option being logged");
        } else {
            match self.command[1..].join(" ").as_str() {
                "-h" => {
                    final_output.push_str("\n");
                    final_output.push_str(format!("Command count in history: {}", self.history.len()).as_str());
                    for command in &self.history {
                        final_output.push_str("\n");
                        final_output.push_str(&command);
                    }
                }
                _ => {
                    final_output.push_str("Invalid arguments");
                }
            }
        }
        self.output.push(final_output.to_string());
    }
}