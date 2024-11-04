use crate::Terminal;

impl Terminal {
    pub fn shout(&mut self, final_output: &mut String) {
        final_output.push('\n');
        final_output.push_str(self.command[1..].join(" ").as_str());
        self.output.push(final_output.to_string());
    }
}