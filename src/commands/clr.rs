use crate::Terminal;

impl Terminal {
    // Clear the output screen
    pub fn clr(&mut self) {
        self.output = Vec::new();
    }
}