use iced::{Alignment, Length, widget::{Column, Space}};

use crate::screen::Screen;

pub struct HelpScreen;

impl Screen for HelpScreen {
    fn view<'a>(&'a self, app: &'a crate::NbtEdit) -> iced::Element<'a, crate::AppMessage, iced::Theme> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push("Help")
            .push(Space::new(Length::Fill, Length::Fill))
            .align_items(Alignment::Center)
            .into()
    }
}