use std::borrow::Cow;

use iced::{widget::{Button, Text}, alignment::Horizontal, Length};

use crate::AppMessage;

#[macro_export]
macro_rules! is_dir {
    ($self: expr, $t: expr) => {
        if let Some(_) = $self.directory {
            Some($t)
        } else {
            None
        }
    };
}

pub fn btn_centered<'a>(text: impl Into<Cow<'a, str>>, width: impl Into<Length>) -> Button<'a, AppMessage> {
    Button::new(Text::new(text).horizontal_alignment(Horizontal::Center)).padding(4).width(width)
}