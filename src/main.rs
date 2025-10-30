use crate::drawer::drawer;
use crate::new_task::{NewTaskMessage, NewTaskState};
use crate::Message::{NullMessage, ToggleNewTaskForm};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill, Size, Theme};

mod new_task;
mod drawer;

fn main() -> iced::Result {
    iced::application("Todo App!", TodoAppState::update, TodoAppState::view)
        .theme(|_| Theme::Dark)
        .font(iced_aw::iced_fonts::REQUIRED_FONT_BYTES)
        .run()
}

#[derive(Default)]
pub struct TodoAppState {
    pub window_size: Size,
    pub show_new_task_form: bool,
    pub new_task_state: NewTaskState,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleNewTaskForm,
    NewTaskMessage(NewTaskMessage),
    NullMessage,
}

impl TodoAppState {
    pub fn view(&self) -> Element<'_, Message> {
        let controls = Self::get_controls();

        let container = container(text("foo bar"))
            .width(Fill)
            .height(Fill);

        let drawer = drawer(
            self.show_new_task_form,
            ToggleNewTaskForm,
            || self.new_task_state.view().map(map_new_task_message),
            container,
            );

        column![
            controls, drawer,
        ].into()
    }

    fn get_controls<'a>() -> Element<'a, Message> {
        let new_task_button = button("New Task")
            .style(button::success)
            .on_press(ToggleNewTaskForm);

        row![new_task_button]
            .padding(5)
            .width(Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ToggleNewTaskForm => {
                self.show_new_task_form = !self.show_new_task_form;
            }
            Message::NewTaskMessage(message) => {
                self.new_task_state.update(message)
            },
            NullMessage => println!("Null Message"),
            _ => println!("Unknown message")
        }
    }
}

fn map_new_task_message(message: NewTaskMessage) -> Message {
    match message {
        NewTaskMessage::CancelNewTask => Message::ToggleNewTaskForm,
        _ => Message::NewTaskMessage(message),
    }
}
