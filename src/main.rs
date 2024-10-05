use std::ops::Deref;
use std::sync::Arc;

use iced::widget::text_editor::{Action, Edit, Motion};
use iced::widget::{column, row, text, text_input};
use iced::{Element, Task};

fn main() -> Result<(), iced::Error> {
    iced::application("RUSH Terminal", Terminal::update, Terminal::view).run_with(Terminal::new)
}

#[derive(Default, Clone)]
struct Terminal {
    content: String,
    history: Vec<String>,
    command: Vec<String>,
    current_path: String,
    lines: i32
}

#[derive(Debug, Clone)]
enum Message {
    Edit(String),
    Submit(String)
}

impl Terminal {

    fn new() -> (Self, Task<Message>) {
        (
            Self {
                content: String::new(),
                history: vec!["".to_string()],
                command: Vec::new(),
                current_path: "home".to_string(),
                lines: 1
            },
            text_input::focus(text_input::Id::new("terminal_input"))
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(content) => {
                self.content = content
            }
            Message::Submit(content) => {
                if content.len() != 0 {
                    self.history.push(content.clone());
                    self.command = content.split_terminator(' ').map(|t| t.to_string()).collect();
                    match self.command[0].as_str() {
                        "shout" => {
                            println!("{}", self.command[1..].join(" "))
                        }
                        _ => println!("Invalid command")
                    }
                }
                self.lines += 1
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column((0..self.lines)
            .map(
                |i| row![
                    text(">")
                    .size(32), 
                    text_input("", &self.history[i as usize])
                    .on_input(Message::Edit)
                    .on_submit(Message::Submit(self.content.clone()))
                    .id("terminal_input")
                ].into()
            )
        )
        .into()
    }
}