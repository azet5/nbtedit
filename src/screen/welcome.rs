use iced::{Alignment, Length, widget::{Column, Space}};

use crate::screen::Screen;

pub struct WelcomeScreen;

impl Screen for WelcomeScreen {
    fn view<'a>(&'a self, _app: &'a crate::NbtEdit) -> iced::Element<'a, crate::AppMessage, iced::Theme> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push("Welcome!")
            .push("To start editing, open a directory with Minecraft save.")
            .push(Space::new(Length::Fill, Length::Fill))
            .align_items(Alignment::Center)
            .into()
    }
}