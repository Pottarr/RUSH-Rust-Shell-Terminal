use crate::Terminal;

impl Terminal {
    pub fn piping_helper(&mut self, final_output: &mut String, all_contents: &Vec<(String, String)>) {
        let index_of_pipe = self.command.iter().position(|x| x == "|").unwrap();
        match self.command[index_of_pipe..].len() {
            1 => {
                final_output.push('\n');
                final_output.push_str("Error: pipe to nothing");
                } 
            2 => {
                match self.command[index_of_pipe+1].clone().as_str() {
                    "find" => {
                        final_output.push('\n');
                        final_output.push_str("Please enter a phrase to find");
                    }
                    _ => {
                        final_output.push('\n');
                        final_output.push_str(format!("Can't pipe with command: {}", self.command[index_of_pipe+1].clone()).as_str());
                    }
                }
            }
            3 => {
                let phrase = String::from(self.command[index_of_pipe + 2].clone());
                match self.command[index_of_pipe+1].clone().as_str() {
                    "find" => {
                        for file in all_contents {
                            let mut found = false;
                            for line in file.1.split('\n') {
                                if line.contains(&phrase) {
                                    found = true;
                                    final_output.push('\n');
                                    final_output.push_str(format!("{}: {}", file.0, line).as_str());
                                }
                            }
                            if !found {
                                final_output.push('\n');
                                final_output.push_str(format!("Phrase \"{}\" not found in {}", &phrase, file.0).as_str());
                            }
                        }
                    }
                    _ => {
                        final_output.push('\n');
                        final_output.push_str(format!("Can't pipe with command: {}", self.command[index_of_pipe+1].clone()).as_str());
                    }
                }
            }
            _ => {
                match self.command[index_of_pipe+1].clone().as_str() {
                    "find" => {
                        final_output.push('\n');
                        final_output.push_str("Too many number of phrase arguments to find");
                    }
                    _ => {
                        final_output.push('\n');
                        final_output.push_str(format!("Can't pipe with command: {}", self.command[index_of_pipe+1].clone()).as_str());
                    }
                }
            }
        }
        self.output.push(final_output.to_string());
    }
}