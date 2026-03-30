use iced::{Alignment, Length, widget::{Column, Space}};

use crate::screen::Screen;

pub struct SettingsScreen;

impl Screen for SettingsScreen {
    fn view<'a>(&'a self, app: &'a crate::NbtEdit) -> iced::Element<'a, crate::AppMessage, iced::Theme> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push("Settings")
            .push(Space::new(Length::Fill, Length::Fill))
            .align_items(Alignment::Center)
            .into()
    }
}