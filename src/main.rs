use crate::drawer::drawer;
use crate::new_task::{NewTaskMessage, NewTaskState};
use crate::Message::{NullMessage, ToggleNewTaskForm, WindowResized};
use iced::widget::{button, container, row};
use iced::{Element, Fill, Size, Subscription, Theme};

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
    WindowResized(Size),
    NullMessage,
}

impl TodoAppState {
    pub fn view(&self) -> Element<'_, Message> {
        let controls = Self::get_controls();
        let container = container(controls)
            .width(Fill)
            .height(Fill);

        drawer(
            self.show_new_task_form,
            ToggleNewTaskForm,
            || self.new_task_state.view().map(map_new_task_message),
            self.window_size,
            container,
            )
            .into()
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
            WindowResized(size) => {
                self.window_size = size;
            },
            NullMessage => println!("Null Message"),
            _ => println!("Unknown message")
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|e, _, _| match e {
            iced::Event::Window(iced::window::Event::Resized(size)) => {
                Some(WindowResized(size))
            }
            _ => None,
        })
    }
}

fn map_new_task_message(message: NewTaskMessage) -> Message {
    match message {
        NewTaskMessage::CancelNewTask => Message::ToggleNewTaskForm,
        _ => Message::NewTaskMessage(message),
    }
}
