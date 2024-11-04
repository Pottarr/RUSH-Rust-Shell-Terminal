use std::path::PathBuf;
use std::process;
use home::home_dir;

use iced::widget::{column, pick_list, row, scrollable, text, text_input};
use iced::{event, Element, Event, Subscription, Task, Theme};

mod commands;

fn main() -> iced::Result {
    iced::application("RUSH Terminal", Terminal::update, Terminal::view)
    .theme(Terminal::theme)
    .subscription(Terminal::subscription)
    .run_with(Terminal::new)
}

#[derive(Default)]
struct Terminal {
    theme: Theme,
    content: String,
    history: Vec<String>,
    command: Vec<String>,
    output: Vec<String>,
    current_path: PathBuf
}

#[derive(Debug, Clone)]
enum Message {
    Edit(String),
    Submit(String),
    ChangeTheme(Theme),
    ViewHistory(Event)
}

impl Terminal {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                theme: Theme::CatppuccinFrappe,
                content: String::new(),
                history: Vec::new(),
                command: Vec::new(),
                output: vec!["Welcome to RUSH!".to_string()],
                current_path: home_dir().unwrap()
            },
            text_input::focus("input")
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit(content) => {
                self.content = content;
                Task::none()
            }
            Message::Submit(content) => {
                if content.trim().len() != 0 {
                    self.history.push(content.clone());
                    self.command = content.trim().split_terminator(' ').map(|t| t.to_string()).collect();
                    let mut final_output = format!("{}$ {}", self.current_path.to_str().unwrap().replace(home_dir().unwrap().to_str().unwrap(), "~"),content);

                    match self.command[0].as_str() {
                        "shout" => self.shout(&mut final_output),
                        "clr" => self.clr(),
                        "cd" => self.cd(&mut final_output),
                        "ls" => self.ls(&mut final_output),
                        "mkfile" => self.mkfile(&mut final_output),
                        "mkdir" => self.mkdir(&mut final_output),
                        "meow" => self.meow(&mut final_output),
                        "exit" => process::exit(0),
                        _ => {
                            final_output.push('\n');
                            final_output.push_str("Invalid command");
                            self.output.push(final_output.to_string())
                        }
                    }
                } else {
                    self.output.push("$".to_string());
                }
                self.content = String::new();
                Task::none()
            }
            Message::ChangeTheme(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::ViewHistory(event) => {
                // println!("{:?}", event);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Change theme:").size(20), pick_list(Theme::ALL, Some(&self.theme), Message::ChangeTheme)],
            row![
                text(format!("user@rush {}$", self.current_path.to_str().unwrap().replace(home_dir().unwrap().to_str().unwrap(), "~"))).size(34),
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

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::ViewHistory)
    }
}