use crate::Terminal;
use std::io::Read;
use std::fs::File;
use std::path::Path;

impl Terminal {
    // find phrase in a file, you can find phrase in multiple files using piping
    pub fn find(&mut self, final_output: &mut String) {
        if self.command.len() == 1 {
            final_output.push('\n');
            final_output.push_str("Please enter a phrase to find in a file");
            self.output.push(final_output.to_string());
        } else if self.command.len() == 2 {
            final_output.push('\n');
            final_output.push_str("Please enter a file name to find the phrase");
            self.output.push(final_output.to_string());
        } else {
            let phrase = String::from(self.command[1].clone());     
            let mut words: Vec<String> = Vec::new();    // This will be used for final output, stores all occurence of the phrase
            let mut contents = String::new();   // Store the whole file content
            for path in &self.command[2..] {
                let mut file: File;
                // if the filename has a root, use that as the path
                if Path::new(path).has_root() {
                    file = match File::open(Path::new(path)) {
                        Ok(f) => f,
                        Err(_) => {
                            final_output.push('\n');
                            final_output.push_str(format!("{path}: File doesn't exist").as_str());
                            continue
                        }
                    };
                } else {
                    // if no root, add a root into the path to prevent errors
                    file = match File::open(Path::new(format!("{}/{path}", self.current_path.to_str().unwrap()).as_str())) {
                        Ok(f) => f,
                        Err(_) => {
                            final_output.push('\n');
                            final_output.push_str(format!("{path}: File doesn't exist").as_str());
                            continue
                        }
                    };
                }
                // Usually, the read_to_string method reads the file contents and stores in a String and returns Result
                // but since our function doesn't need to return anything, we use a match case to handle a result
                // and don't do anything about it
                match file.read_to_string(&mut contents) {
                    Ok(_) => println!("File read to string successfully"),
                    Err(e) => println!("Error at File read to string: {}", e)
                }
                let temp: Vec<String> = contents.split('\n').map(|x| x.to_string()).collect();
                // find the phrases in a contents String which we split on a newline char
                for sentence in &temp {
                    if sentence.contains(&phrase) {
                        words.push(sentence.clone());
                    }
                }
                if words.is_empty() {
                    final_output.push('\n');
                    final_output.push_str(format!("Phrase \"{}\" not found in {}", &phrase, path).as_str());
                } else {
                    for word in &words {
                        final_output.push('\n');
                        final_output.push_str(format!("{}: {}", path, word).as_str());
                    }
                }
                words.clear();
                contents.clear();
            }
            self.output.push(final_output.to_string());
        }
        self.current_command_position = self.history.len()
    }
}