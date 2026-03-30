pub mod open;
pub mod edit;
pub mod save;
pub mod welcome;
pub mod settings;
pub mod help;

use std::fmt::Debug;

use iced::Theme;

use crate::{AppMessage, NbtEdit};

#[derive(Debug, Clone)]
pub enum ScreenTy {
    Welcome,
    Open,
    Save,
    Edit,
    Settings,
    Help,
}

impl ScreenTy {
    pub fn get_screen(&self) -> Box<dyn Screen> {
        match self {
            ScreenTy::Welcome => todo!(),
            ScreenTy::Open => Box::new(open::OpenScreen),
            ScreenTy::Save => Box::new(save::SaveScreen),
            ScreenTy::Edit => Box::new(edit::EditScreen),
            ScreenTy::Settings => Box::new(settings::SettingsScreen),
            ScreenTy::Help => Box::new(help::HelpScreen),
        }
    }
}

pub trait Screen {
    fn view<'a>(&'a self, app: &'a NbtEdit) -> iced::Element<'a, AppMessage, Theme>;
    // fn update(&mut self, message: AppMessage);
}

impl Debug for dyn Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("trait Screen")
    }
}
