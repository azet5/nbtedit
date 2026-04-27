use iced::{Length, widget::{Button, Column, Row, Rule, Scrollable, Text}};

use crate::{NbtEdit, helpers::{CurrentPath, dir_buttons}, screen::Screen};

pub struct OpenScreen;

fn btn_breadcrumb<'a>(path: &CurrentPath) -> iced::Element<'a, crate::AppMessage, iced::Theme> {
    let mut row = Row::new()
        .padding(4)
        .spacing(0);

    for element in path {
        row = row.push(Button::new(Text::new(element.0)).on_press(crate::AppMessage::ChangeDir(element.1.clone())));
    }

    row.into()
}

impl Screen for OpenScreen {
    fn view<'a>(&'a self, app: &'a NbtEdit) -> iced::Element<'a, crate::AppMessage, iced::Theme> {
        let path = app.path.get();
        Column::new()
            .push(Row::new()
                .push(btn_breadcrumb(&app.path))
            )
            .push(Rule::horizontal(1))
            .push(Row::new()
                // .push(Scrollable::new(default_paths()))
                .push(Scrollable::new(dir_buttons(path)).width(Length::Fill))
            )
            .height(Length::Fill)
            .spacing(4)
            .padding(4)
            // .align_items(Alignment::Center)
            .into()
    }

    fn get_type<'a>(&'a self) -> super::ScreenTy {
        super::ScreenTy::Open
    }
}