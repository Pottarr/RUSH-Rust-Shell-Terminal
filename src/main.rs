use std::ops::Deref;
use std::sync::Arc;

use iced::widget::text_editor::{Action, Edit, Motion};
use iced::widget::{column, pick_list, row, scrollable, text, text_editor, text_input};
use iced::{Element, Task, Theme};

fn main() -> Result<(), iced::Error> {
    iced::application("RUSH Terminal", Terminal::update, Terminal::view)
    .theme(Terminal::theme)
    .run_with(Terminal::new)
}

#[derive(Default)]
struct Terminal {
    theme: Theme,
    content: String,
    history: Vec<String>,
    command: Vec<String>,
    output: Vec<String>,
    current_path: String
}

#[derive(Debug, Clone)]
enum Message {
    Edit(String),
    Submit(String),
    ChangeTheme(Theme)
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
                current_path: "~".to_string()
            },
            text_input::focus("input")
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(content) => {
                self.content = content
            }
            Message::Submit(content) => {
                if content.trim().len() != 0 {
                    self.history.push(content.clone());
                    self.command = content.trim().split_terminator(' ').map(|t| t.to_string()).collect();

                    match self.command[0].as_str() {
                        "shout" => self.output.push(self.command[1..].join(" ")),
                        _ => self.output.push("Invalid command".to_string())
                    }
                } else {
                    self.output.push("".to_string());
                }
                self.content = String::new()
            }
            Message::ChangeTheme(theme) => self.theme = theme
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Change theme:").size(20), pick_list(Theme::ALL, Some(&self.theme), Message::ChangeTheme)],
            row![
                text(format!("user@rush {}$", self.current_path)).size(34),
                text_input("", &self.content).size(30)
                .on_input(Message::Edit)
                .on_submit(Message::Submit(self.content.clone()))
                .id("input")
            ].width(1000),
            scrollable(
                column((0..self.output.len()).map(|i| text(format!("> {}", self.output[i].to_string())).size(26).into()))
            ).width(1000)
        ]
        .spacing(30)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

