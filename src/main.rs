use crate::new_task::{NewTaskMessage, NewTaskState};
use crate::Message::NullMessage;
use iced::widget::{button, column, row};
use iced::{Element, Fill, Theme};

mod new_task;

fn main() -> iced::Result {
    iced::application("Todo App!", TodoAppState::update, TodoAppState::view)
        .theme(|_| Theme::Dark)
        .font(iced_aw::iced_fonts::REQUIRED_FONT_BYTES)
        .run()
}

#[derive(Default)]
pub struct TodoAppState {
    pub show_new_task_form: bool,
    pub new_task_state: NewTaskState,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleNewTaskForm(bool),
    NewTaskMessage(NewTaskMessage),
    NullMessage,
}

impl TodoAppState {
    pub fn view(&self) -> Element<'_, Message> {

        let controls = Self::get_controls();

        if self.show_new_task_form {
            column![controls, self.new_task_state.view().map(map_new_task_message)]
                .into()
        } else {
            controls
        }
    }

    fn get_controls<'a>() -> Element<'a, Message> {
        let new_task_button = button("New Task")
            .style(button::success)
            .on_press(Message::ToggleNewTaskForm(true));

        row![new_task_button]
            .padding(5)
            .width(Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleNewTaskForm(show_new_task_form) => {
                self.show_new_task_form = show_new_task_form;
            }
            Message::NewTaskMessage(message) => {
                self.new_task_state.update(message)
            }
            NullMessage => println!("Null Message"),
            _ => println!("Unknown message")
        }
    }
}

fn map_new_task_message(message: NewTaskMessage) -> Message {
    match message {
        NewTaskMessage::CancelNewTask => Message::ToggleNewTaskForm(false),
        _ => Message::NewTaskMessage(message),
    }
}
