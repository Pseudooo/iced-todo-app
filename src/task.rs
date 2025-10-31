use crate::task::TaskMessage::{DescriptionUpdated, TitleUpdated, ToggleCompleted, ToggleIsEditing};
use iced::widget::{button, checkbox, column, container, row, text, text_input};
use iced::{Center, Element, Fill, Theme};
use iced_aw::date_picker::Date;
use lucide_icons::iced::icon_pencil;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskState {
    pub id: Uuid,
    pub completed: bool,
    pub is_editing: bool,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<Date>,
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    ToggleCompleted(bool),
    ToggleIsEditing,
    TitleUpdated(String),
    DescriptionUpdated(String),
}

impl TaskState {
    pub fn new(title: String, description: Option<String>, due_date: Option<Date>) -> Self {
        Self {
            id: Uuid::new_v4(),
            completed: false,
            is_editing: false,
            title,
            description,
            due_date,
        }
    }

    pub fn view(&self) -> Element<'_, TaskMessage> {
        let completed_toggle = checkbox("", self.completed)
            .on_toggle(ToggleCompleted);
        let edit_button = button(icon_pencil())
            .style(button::success)
            .on_press(ToggleIsEditing);
        let controls = row![completed_toggle, edit_button]
            .align_y(Center)
            .spacing(5);

        let details = self.get_details_section();

        let row = row![
                controls,
                details,
            ]
            .align_y(Center)
            .spacing(10);
        container(row)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    border: iced::border::width(2)
                        .rounded(5)
                        .color(palette.secondary.base.color),
                    ..Default::default()
                }
            })
            .width(Fill)
            .padding(10)
            .into()
    }

    fn get_details_section(&self) -> Element<'_, TaskMessage> {
        if !self.is_editing {
            let title_label = text(self.title.as_str());
            let description_label = text(self.description.clone().unwrap_or("".to_string()));
            return column![
                    title_label,
                    description_label,
                ]
                .into()
        }

        let title_input = text_input("Title", self.title.as_str())
            .on_input(TitleUpdated);
        let description_input = text_input("Description", self.description.clone().unwrap_or("".to_string()).as_str())
            .on_input(DescriptionUpdated);
        column![
                title_input,
                description_input,
            ]
            .into()
    }

    pub fn update(&mut self, message: TaskMessage) {
        match message {
            ToggleCompleted(state) => {
                self.completed = state;
            },
            ToggleIsEditing => {
                self.is_editing = !self.is_editing;
            },
            TitleUpdated(title) => {
                self.title = title;
            },
            DescriptionUpdated(description) => {
                if description.is_empty() {
                    self.description = None;
                }
                self.description = Some(description);
            }
        }
    }
}