use crate::Terminal;

impl Terminal {
    pub fn clr(&mut self) {
        self.output = Vec::new();
    }
}