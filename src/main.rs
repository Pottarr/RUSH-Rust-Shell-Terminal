use std::ops::Deref;
use std::sync::Arc;

use iced::widget::text_editor::{Action, Edit, Motion};
use iced::widget::{column, row, text, text_editor};
use iced::Element;

fn main() -> Result<(), iced::Error> {
    iced::application("RUSH Terminal", Terminal::update, Terminal::view).run()
}

#[derive(Default)]
struct Terminal {
    content: text_editor::Content,
    history: Vec<String>,
    command: Vec<String>,
    prompts: Vec<String>
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Terminal {
    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                match action {
                    Action::Edit(Edit::Enter) => {
                        self.history.push(self.content.line(self.content.line_count() - 1).unwrap().deref().to_string());
                        if self.content.line(self.content.line_count() - 1).unwrap().len() == 0 {
                            self.command = vec!["".to_string()]
                        } else {
                            self.command = self.content.line(self.content.line_count() - 1).unwrap().split_terminator(' ').map(|v| v.to_string()).collect::<Vec<String>>();
                        }
                        self.content.perform(action);
                        match self.command[0].as_str() {
                            "shout" => {
                                self.content.perform(Action::Edit(Edit::Paste(Arc::new(format!("{}\n", self.command[1..].join(" "))))));
                                self.prompts.push("".to_string())
                            }
                            _ => {}
                        }
                        self.prompts.push(">".to_string())
                    }
                    Action::Edit(Edit::Backspace) => {
                        if self.content.line(self.content.line_count() - 1).unwrap().len() != 0 {
                            self.content.perform(action);
                        }
                    }
                    Action::Move(motion) => {
                        match motion {
                            Motion::Up | Motion::Down => {}
                            _ => self.content.perform(action)
                        }
                    }
                    _ => self.content.perform(action)
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            column![
                text(">").size(32), 
                column((self.prompts.iter()).map(|t| text(t.to_string()).size(32).into()))
            ],
            text_editor(&self.content)
            .size(32)
            .on_action(Message::Edit)
        ]
        .spacing(5)
        .into()
    }
}

