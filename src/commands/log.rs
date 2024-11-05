use crate::Terminal;

impl Terminal {
    // Print out all the command history
    pub fn log(&mut self, final_output: &mut String) {
        final_output.push_str("\n");
        final_output.push_str(format!("Command count in history: {}", self.history.len()).as_str());
        for command in &self.history {
            final_output.push_str("\n");
            final_output.push_str(&command);
        }
        self.output.push(final_output.to_string());
    }
}
