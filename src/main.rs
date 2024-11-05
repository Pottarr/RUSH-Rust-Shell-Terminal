use std::path::PathBuf;
use std::process;
use home::home_dir;

use iced::widget::{column, pick_list, row, scrollable, text, text_input};
use iced::{color, event, Element, Event, Subscription, Task, Theme};

mod commands;
mod services;

// main function
fn main() -> iced::Result {
    iced::application("RUSH Terminal", Terminal::update, Terminal::view)
    .theme(Terminal::theme)
    .subscription(Terminal::subscription)
    .run_with(Terminal::new)
}

// This will be the struct used in the program
#[derive(Default)]
struct Terminal {
    theme: Theme,
    content: String,    // This String stored what you just entered in the entry box
    history: Vec<String>,   // Command history
    command: Vec<String>,   // Takes the content String and split into vector for easier command matching
    output: Vec<String>,    // The final output into the output panel
    current_command_position: usize,    // For the history management
    current_path: PathBuf,  // Current working directory
}

// This will be for detecting any kind of events that happened in the program
// example: when user type something, when user hit the Enter key
#[derive(Debug, Clone)]
enum Message {
    Edit(String),       // Use for detect typing events
    Submit(String),     // Use for when user hits enter button
    ChangeTheme(Theme), // When user change theme
    ViewHistory(Event)  // For pressing the up and down keys to reuse history
}

impl Terminal {
    // Create new data
    fn new() -> (Self, Task<Message>) {
        let mut initial_setup = Self {
            theme: Theme::CatppuccinFrappe,
            content: String::new(),
            history: Vec::new(),
            command: Vec::new(),
            output: vec!["Welcome to RUSH!".to_string()],
            current_command_position: 0,
            current_path: home_dir().unwrap(),
        };
        initial_setup.startup_sync_history();

        (
            initial_setup,
            text_input::focus("input")  // This one is use for focusing on the entry box
        )                                   // So user don't have to click the entry box every time
    }                                      
    
    // The main update function that will perform actions based on what we do
    fn update(&mut self, message: Message) -> Task<Message> {
        match message { // if we type anything, the content String changes
            Message::Edit(content) => {
                self.content = content;
                Task::none()
            }
            Message::Submit(content) => {   // When user hit the Enter key
                if &content.trim().len() != &0 {
                    self.push_history(&content);
                    // splitting the content String into a vec and store in self.command
                    self.command = content.trim().split_terminator(' ').map(|t| t.to_string()).collect();
                    // Initializing the final output to print out to the output
                    let mut final_output = format!("{}$ {}", self.current_path.to_str().unwrap().replace(home_dir().unwrap().to_str().unwrap(), "~"),content);

                    // matching the first item in the self.command vec, which is basically a command,
                    // and execute each corresponding functions which we have wrote seperately
                    // and store in the commands folder
                    match self.command[0].as_str() {
                        "shout" => self.shout(&mut final_output),
                        "clr" => self.clr(),
                        "cd" => self.cd(&mut final_output),
                        "ls" => self.ls(&mut final_output),
                        "mkfile" => self.mkfile(&mut final_output),
                        "mkdir" => self.mkdir(&mut final_output),
                        "log" => self.log(&mut final_output),
                        "meow" => self.meow(&mut final_output),
                        "find" => self.find(&mut final_output),
                        "help" => self.help(&mut final_output),
                        "exit" => process::exit(0),
                        _ => { // if none match, output the default option
                            final_output.push('\n');
                            final_output.push_str("Invalid command");
                            self.output.push(final_output.to_string())
                        }
                    }
                } else { // if the user typed nothing and just press Enter, output the directory name only
                    self.output.push(format!("{}$", self.current_path.to_str().unwrap()));
                }
                self.content = String::new();
                Task::none()
            }
            Message::ChangeTheme(theme) => {    // changing theme
                self.theme = theme;
                Task::none()
            }
            Message::ViewHistory(event) => {    // detect up and down key presses and view history
                self.history(event);
                Task::none()
            }
        }
    }

    // Main GUI components
    fn view(&self) -> Element<Message> {
        let path_str;
        if cfg!(target_os = "windows") {
            path_str = format!("user@rush {}$", self.current_path.to_str().unwrap().replace(home_dir().unwrap().to_str().unwrap(), "~").replace("/", r"\"));
        } else {
            path_str = format!("user@rush {}$", self.current_path.to_str().unwrap().replace(home_dir().unwrap().to_str().unwrap(), "~"));
        }
        column![
            row![text("Change theme:").size(20), pick_list(Theme::ALL, Some(&self.theme), Message::ChangeTheme)],
            row![
                text(path_str).size(34).color(color!(0x00e03c)),
                text_input("", &self.content).size(30)
                .on_input(Message::Edit)
                .on_submit(Message::Submit(self.content.clone()))
                .id("input")
            ],
            scrollable(
                column((0..self.output.len()).map(|i| text(format!("{}", self.output[i].to_string())).size(26).into()))
            ).width(1000)
        ]
        .padding(4)
        .spacing(30)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    // This function is used for detecting any other key presses
    // other than the english letters and send it as the ViewHistory event
    // example: the arrow keys, F1-F12, Shift, Tab, Ctrl, etc.
    fn subscription(&self) -> Subscription<Message> { 
        event::listen().map(Message::ViewHistory)
    }
}