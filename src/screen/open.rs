use iced::{Length, widget::Column};

use crate::{NbtEdit, screen::Screen};

pub struct OpenScreen;

impl Screen for OpenScreen {
    fn view<'a>(&'a self, app: &'a NbtEdit) -> iced::Element<'a, crate::AppMessage, iced::Theme> {
        Column::new()
            .push(app.dir_viewer.view())
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