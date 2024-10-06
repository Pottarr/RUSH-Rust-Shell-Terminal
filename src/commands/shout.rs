use crate::Terminal;

impl Terminal {
    pub fn shout(&mut self) {
        self.output.push(self.command[1..].join(" "));
    }
}