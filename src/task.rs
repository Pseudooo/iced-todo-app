use crate::task::TaskMessage::{DescriptionUpdated, TitleUpdated, ToggleCompleted, ToggleIsEditing};
use iced::widget::{button, checkbox, column, container, row, text, text_input};
use iced::{Center, Element, Fill, Font, Theme};
use iced::font::{Style, Weight};
use iced_aw::date_picker::Date;
use lucide_icons::iced::{icon_check, icon_pencil};
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
        let controls = self.get_controls();
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
                let border_color = match self.is_editing {
                    true => palette.primary.strong.color,
                    false => palette.secondary.base.color,
                };

                container::Style {
                    border: iced::border::width(2)
                        .rounded(5)
                        .color(border_color),
                    ..Default::default()
                }
            })
            .width(Fill)
            .padding(10)
            .into()
    }

    fn get_controls(&self) -> Element<'_, TaskMessage> {
        let completed_toggle = checkbox("", self.completed)
            .on_toggle(ToggleCompleted);
        let edit_button = match self.is_editing {
            true => button(icon_check()),
            false => button(icon_pencil()),
        }.on_press(ToggleIsEditing).style(button::success);

        row![completed_toggle, edit_button]
            .align_y(Center)
            .spacing(5)
            .into()
    }

    fn get_details_section(&self) -> Element<'_, TaskMessage> {
        if !self.is_editing {
            let title_label = container(text(self.title.as_str())
                .font(Font {
                    weight: Weight::Bold,
                    ..Default::default()
                }))
                .padding(5);
            let description_label = container(text(self.description.clone().unwrap_or("".to_string()))
                .font(Font {
                    style: Style::Italic,
                    ..Default::default()
                }))
                .padding(5);
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