use crate::Terminal;

impl Terminal {
    // print the contents to the output
    // shout can also use redirect
    pub fn shout(&mut self, final_output: &mut String) {
        let mut contents: String = String::new();
        final_output.push('\n');
        for word in &self.command[1..] {
            if word == ">" || word == ">>" {
                break
            }
            final_output.push_str(format!("{word} ").as_str());
            contents.push_str(format!("{} ", word).as_str());
        }
        if self.command.contains(&">".to_string()) || self.command.contains(&">>".to_string()) {
            self.redirect_helper(final_output, contents);
        } else {
            self.output.push(final_output.to_string());
        }
        self.current_command_position = self.history.len()
    }
}