use iced::{Alignment, Length, widget::{Button, Column, Row, Scrollable, Text}};

use crate::{AppMessage, helpers::{btn_centered, labeled_element}, queue::ActionType, screen::Screen};

pub struct SaveScreen;

impl Screen for SaveScreen {
    fn view<'a>(&'a self, app: &'a crate::NbtEdit) -> iced::Element<'a, crate::AppMessage, iced::Theme> {
        let mut buttons = Column::new().padding(4).spacing(4);
        let mut options = Column::new().width(Length::FillPortion(2)).padding(4).spacing(4);
        
        if app.queue.length() > 0 {
            for item in app.queue.iter() {
                let text = match item {
                    ActionType::Add { id, .. } => format!(
                        "add: {}",
                        app.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap().name().unwrap_or(&"(empty)".to_string())
                    ),
                    ActionType::Delete(id) => format!(
                        "delete: {}",
                        app.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap().name().unwrap_or(&"(empty)".to_string())
                    ),
                    ActionType::Edit { id, .. } => format!(
                        "edit: {}",
                        app.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap().name().unwrap_or(&"(empty)".to_string())
                    ),
                };
                
                buttons = buttons.push(btn_centered(text, Length::Fill).on_press(AppMessage::InputAction(item.clone())));
            }
        }
        
        if let Some(action) = &app.selected_action {
            match action {
                ActionType::Add { id, .. } => {
                    let tag = app.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap();
                    options = options.push(labeled_element("type", Text::new(format!("{}", tag.get())).into()));
                },
                ActionType::Edit { id, old_name, .. } => {
                    let tag = app.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap();
                    options = options.push(labeled_element("type", Text::new(format!("{}", tag.get())).into()));
                    options = options.push(labeled_element("old key", Text::new(old_name).into()));
                }
                ActionType::Delete(id) => {
                    let tag = app.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap();
                    options = options.push(labeled_element("Type:", Text::new(tag.get().type_name()).into()));
                    options = options.push(labeled_element("Value:", Text::new(tag.get().to_string()).into()));
                },
            }
        }

        Column::new()
            .push(Row::new()
                .push(Button::new("Apply").on_press_maybe(if app.queue.length() > 0 {
                    Some(AppMessage::Write)
                } else { None }))
                .push(Text::new(format!("{} action(s)", app.queue.length())))
                .padding(4)
                .spacing(4)
                .align_items(Alignment::Center)
            ).push(Row::new()
                .push(Scrollable::new(buttons).width(Length::FillPortion(2)))
                .push(options)
                .height(Length::Fill)
            ).padding(4)
            .spacing(4)
            .into()
    }
}