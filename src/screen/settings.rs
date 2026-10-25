use iced::{Alignment, widget::Column};

use crate::screen::Screen;

pub struct SettingsScreen;

impl Screen for SettingsScreen {
    fn view<'a>(&'a self, app: &'a crate::NbtEdit) -> iced::Element<'a, crate::AppMessage, iced::Theme> {
        Column::new()
            .push(app.settings.view())
            .align_x(Alignment::Center)
            .into()
    }

    fn get_type<'a>(&'a self) -> super::ScreenTy {
        super::ScreenTy::Settings
    }
}