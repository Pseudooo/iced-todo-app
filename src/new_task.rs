use crate::new_task::NewTaskMessage::{CancelNewTask, ClearDueDate, CreateNewTask, DescriptionChanged, SubmitDatePicker, TitleChanged, ToggleDatePicker};
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Center, Element, Fill, Left};
use iced_aw::date_picker::Date;
use iced_aw::DatePicker;

#[derive(Default)]
pub struct NewTaskState {
    pub title: String,
    pub description: String,
    pub due_date: Option<Date>,
    pub show_date_picker: bool,
}

#[derive(Debug, Clone)]
pub struct NewTaskPayload {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<Date>,
}

#[derive(Debug, Clone)]
pub enum NewTaskMessage {
    TitleChanged(String),
    DescriptionChanged(String),
    ToggleDatePicker(bool),
    SubmitDatePicker(Date),
    ClearDueDate,
    CancelNewTask,
    CreateNewTask(NewTaskPayload),
}

impl NewTaskState {
    pub fn view(&self) -> Element<'_, NewTaskMessage> {
        let title_label = text("Create a new task")
            .align_x(Left)
            .size(30);

        let task_title_input = text_input("Title", self.title.as_str())
            .on_input(TitleChanged);

        let task_description_input = text_input("Description", self.description.as_str())
            .on_input(DescriptionChanged);

        let due_date_row: Element<NewTaskMessage> = match self.due_date {
            Some(due_date) => {
                let label = text(due_date.to_string())
                    .size(18);
                let due_date_input = DatePicker::new(
                    self.show_date_picker,
                    Date::today(),
                    button("Change date", )
                        .on_press(ToggleDatePicker(true)),
                    ToggleDatePicker(false),
                    SubmitDatePicker,
                );
                let clear_button = button("Remove")
                    .style(button::danger)
                    .on_press(ClearDueDate);
                row![label, due_date_input, clear_button]
                    .spacing(10)
                    .align_y(Center)
                    .into()
            }
            None => {
                DatePicker::new(
                    self.show_date_picker,
                    Date::today(),
                    button("Add a due date")
                        .on_press(ToggleDatePicker(true)),
                    ToggleDatePicker(false),
                    SubmitDatePicker,
                ).into()
            }
        };

        let save_button= button("Save")
            .style(button::success)
            .on_press(CreateNewTask(NewTaskPayload {
                title: self.title.clone(),
                description: if self.description.is_empty() { None } else { Some(self.description.clone()) },
                due_date: self.due_date,
            }));
        let cancel_button = button("Cancel")
            .style(button::danger)
            .on_press(CancelNewTask);

        let controls = container(row![save_button, cancel_button]
            .spacing(10))
            .align_x(Center)
            .width(Fill);

        container(column![title_label, task_title_input, task_description_input, due_date_row, controls]
            .spacing(10)
            .padding(10))
            .into()
    }

    pub fn update(&mut self, message: NewTaskMessage) {
        match message {
            TitleChanged(new_title) => {
                self.title = new_title;
            }
            DescriptionChanged(new_description) => {
                self.description = new_description;
            },
            ToggleDatePicker(state) => {
                self.show_date_picker = state;
            },
            SubmitDatePicker(due_date) => {
                self.due_date = Some(due_date);
                self.show_date_picker = false;
            }
            ClearDueDate => {
                self.due_date = None;
            },
            CreateNewTask(_) => unreachable!("Create new task will be handled by parent"),
            CancelNewTask => unreachable!("Cancel New task will be intercepted by the parent and handled there"),
        }
    }
}