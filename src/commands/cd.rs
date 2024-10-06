use crate::Terminal;

impl Terminal {
    pub fn cd(&mut self) {
        let dir_change_to = self.command[1..].join(" ");
        self.current_path = dir_change_to.to_string();
        self.output.push(format!("{} {}", "Change Directory to".to_string(), self.current_path));
    }
}