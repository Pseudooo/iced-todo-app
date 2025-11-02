use crate::drawer::drawer;
use crate::new_task::{NewTaskMessage, NewTaskPayload, NewTaskState};
use crate::task::{TaskMessage, TaskState};
use crate::Message::{NullMessage};
use iced::widget::{button, column, keyed_column, row, scrollable};
use iced::{Element, Fill, Size, Theme};
use uuid::Uuid;
use crate::new_task::NewTaskMessage::ClearState;


mod new_task;
mod drawer;
mod task;

fn main() -> iced::Result {
    iced::application("Todo App!", TodoAppState::update, TodoAppState::view)
        .theme(|_| Theme::Dark)
        .font(iced_aw::iced_fonts::REQUIRED_FONT_BYTES)
        .font(lucide_icons::LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Default)]
pub struct TodoAppState {
    pub window_size: Size,
    pub show_new_task_form: bool,
    pub new_task_state: NewTaskState,
    pub tasks: Vec<TaskState>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleNewTaskForm,
    NewTaskMessage(NewTaskMessage),
    TaskMessage(Uuid, TaskMessage),
    CreateNewTask(NewTaskPayload),
    NullMessage,
}

impl TodoAppState {
    pub fn view(&self) -> Element<'_, Message> {
        let controls = Self::get_controls();
        let task_content = self.get_task_list();

        let drawer = drawer(
            self.show_new_task_form,
            Message::ToggleNewTaskForm,
            || self.new_task_state.view().map(map_new_task_message),
            task_content,
            );

        column![
            controls, drawer,
            ]
            .into()
    }

    fn get_controls<'a>() -> Element<'a, Message> {
        let new_task_button = button("New Task")
            .style(button::success)
            .on_press(Message::ToggleNewTaskForm);

        row![new_task_button]
            .padding(5)
            .width(Fill)
            .into()
    }

    fn get_task_list(&self) -> Element<'_, Message> {
        let column = keyed_column(self.tasks.iter()
            .map(|task| {
                let id = task.id;
                (id, task.view().map(move |msg| map_task_message(id, msg)))
            }))
            .padding(10)
            .spacing(10);

        scrollable(column)
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
            Message::CreateNewTask(payload) => {
                let new_task_state = TaskState::new(payload.title, payload.description, payload.due_date);
                self.tasks.push(new_task_state);
                self.show_new_task_form = false;
                self.new_task_state.update(ClearState);
            }
            Message::TaskMessage(id, message) => {
                let target_task = self.tasks.iter_mut()
                    .find(|task| task.id == id);
                target_task.unwrap().update(message)
            }
            NullMessage => println!("Null Message"),
        }
    }
}

fn map_new_task_message(message: NewTaskMessage) -> Message {
    match message {
        NewTaskMessage::CancelNewTask => Message::ToggleNewTaskForm,
        NewTaskMessage::CreateNewTask(payload) => Message::CreateNewTask(payload),
        _ => Message::NewTaskMessage(message),
    }
}

fn map_task_message(id: Uuid, message: TaskMessage) -> Message {
    Message::TaskMessage(id, message)
}
