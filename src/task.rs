use crate::task::TaskMessage::ToggleCompleted;
use iced::widget::{checkbox, column, row, text};
use iced::{Center, Element};
use iced_aw::date_picker::Date;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskState {
    pub id: Uuid,
    pub completed: bool,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<Date>,
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    ToggleCompleted(bool),
}

impl TaskState {
    pub fn view(&self) -> Element<'_, TaskMessage> {
        let completed_toggle = checkbox("", self.completed)
            .on_toggle(ToggleCompleted);

        let title_label = text(self.title.as_str());
        let description_label = text(self.description.clone().unwrap_or("".to_string()));
        let details_column = column![
            title_label,
            description_label,
        ];

        row![
            completed_toggle,
            details_column,
            ]
            .align_y(Center)
            .into()
    }

    pub fn update(&mut self, message: TaskMessage) {
        match message {
            ToggleCompleted(state) => {
                self.completed = state;
            }
        }
    }
}