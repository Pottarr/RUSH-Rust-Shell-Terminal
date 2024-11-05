use crate::Terminal;

impl Terminal {
    pub fn help(&mut self, final_output: &mut String) {
        match self.command.len() {
            1 => {
                final_output.push('\n');
                final_output.push_str("This command will tell the usage of commands.\n");
                final_output.push_str("For example, 'help shout'\n");
                final_output.push_str("All commands:\n");
                final_output.push_str("- cd\n");
                final_output.push_str("- clr\n");
                final_output.push_str("- find\n");
                final_output.push_str("- help\n");
                final_output.push_str("- log\n");
                final_output.push_str("- ls\n");
                final_output.push_str("- meow\n");
                final_output.push_str("- mkdir\n");
                final_output.push_str("- mkfile\n");
                final_output.push_str("- shout\n");
                self.output.push(final_output.to_string());
            }
            2 => {
                match self.command[1].as_str() {
                    "cd" => {
                        final_output.push('\n');
                        final_output.push_str("cd: Change directory\n");
                        final_output.push_str("Usage: cd <path>\n");
                        final_output.push_str("Example: cd /home\n");
                        self.output.push(final_output.to_string());
                    }
                    "clr" => {
                        final_output.push('\n');
                        final_output.push_str("clr: Clear the terminal\n");
                        self.output.push(final_output.to_string());
                    }
                    "find" => {
                        final_output.push('\n');
                        final_output.push_str("find: Find a phrase in a file\n");
                        final_output.push_str("Usage: find <phrase> <file>\n");
                        final_output.push_str("Example: find hello file.txt\n");
                        self.output.push(final_output.to_string());
                    }
                    "help" => {
                        final_output.push('\n');
                        final_output.push_str("help: Display help\n");
                        final_output.push_str("Usage: help <command>\n");
                        final_output.push_str("Example: help cd\n");
                        self.output.push(final_output.to_string());
                    }
                    "log" => {
                        final_output.push('\n');
                        final_output.push_str("log: Show command history\n");
                        self.output.push(final_output.to_string());
                    }
                    "ls" => {
                        final_output.push('\n');
                        final_output.push_str("ls: List files and directories\n");
                        final_output.push_str("Usage: ls <option>\n");
                        final_output.push_str("Example: ls -l\n");
                        self.output.push(final_output.to_string());
                    }
                    "meow" => {
                        final_output.push('\n');
                        final_output.push_str("meow: Concatenate files and print to standard output\n");
                        final_output.push_str("Usage: meow <file1> <file2> <file3> ...\n");
                        final_output.push_str("Example: meow file1.txt file2.txt\n");
                        self.output.push(final_output.to_string());
                    }
                    "mkdir" => {
                        final_output.push('\n');
                        final_output.push_str("mkdir: Create a directory\n");
                        final_output.push_str("Usage: mkdir <directory>\n");
                        final_output.push_str("Example: mkdir new_directory\n");
                        self.output.push(final_output.to_string());
                    }
                    "mkfile" => {
                        final_output.push('\n');
                        final_output.push_str("mkfile: Create a file\n");
                        final_output.push_str("Usage: mkfile <file>\n");
                        final_output.push_str("Example: mkfile new_file.txt\n");
                        self.output.push(final_output.to_string());
                    }
                    "shout" => {
                        final_output.push('\n');
                        final_output.push_str("shout: Print a message\n");
                        final_output.push_str("Usage: shout <message>\n");
                        final_output.push_str("Example: shout hello\n");
                        self.output.push(final_output.to_string());
                    }
                    _ => {
                        final_output.push('\n');
                        final_output.push_str(format!("{}: Invalid command", self.command[1].as_str()).as_str());
                        self.output.push(final_output.to_string());
                    }
                }
            }
            _ => {
                final_output.push('\n');
                final_output.push_str("Too many arguments");
                self.output.push(final_output.to_string());
            }
        }
    }
}