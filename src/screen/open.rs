use iced::{Alignment, Length, widget::{Column, Row, Scrollable}};

use crate::{NbtEdit, helpers::dir_buttons, screen::Screen};

pub struct OpenScreen;

impl Screen for OpenScreen {
    fn view<'a>(&'a self, app: &'a NbtEdit) -> iced::Element<'a, crate::AppMessage, iced::Theme> {
        Column::new()
            .push(Row::new()
                // .push(Scrollable::new(default_paths()))
                .push(Scrollable::new(dir_buttons(&app.path)).width(Length::Fill))
            )
            .height(Length::Fill)
            .spacing(4)
            .padding(4)
            .align_items(Alignment::Center)
            .into()
    }
}