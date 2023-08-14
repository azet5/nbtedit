mod helpers;

use helpers::btn_centered;
use iced::{Sandbox, Settings, window::{self, PlatformSpecific}, widget::{Button, Space, Row, Column, Container}, Length, Alignment};

struct NbtEdit {
    screen: Screen,
    directory: Option<String>
}

#[derive(Debug, Clone)]
pub enum Screen {
    Welcome,
    Open,
    Save,
    Apply,
    Level,
    Player,
    Dim(i32),
    Generic,
    Settings,
    Help,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    ChangeScreen(Screen),
}

impl NbtEdit {
    fn screen_btn(&self, screen: Screen) -> Button<'_, AppMessage> {
        match screen {
            Screen::Open => btn_centered("Open", 60).on_press(AppMessage::ChangeScreen(Screen::Open)),
            Screen::Save => btn_centered("Save", 60).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Save))),
            Screen::Apply => btn_centered("Apply", 60).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Apply))),
            Screen::Level => btn_centered("level.dat", 90).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Level))),
            Screen::Player => btn_centered("player.dat", 90).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Player))),
            Screen::Dim(_) => btn_centered("DIM*.dat", 90).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Dim(0)))),
            Screen::Settings => btn_centered("Settings", 90).on_press(AppMessage::ChangeScreen(Screen::Settings)),
            Screen::Help => btn_centered("?", 30).on_press(AppMessage::ChangeScreen(Screen::Help)),
            _ => unreachable!("no buttons for other types exist"),
        }
    }

    fn welcome(&self) -> iced::Element<'_, AppMessage> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push("Welcome!")
            .push("To start editing, open a directory with Minecraft save.")
            .push(Space::new(Length::Fill, Length::Fill))
            .align_items(Alignment::Center)
            .into()
    }
}

impl Sandbox for NbtEdit {
    type Message = AppMessage;

    fn new() -> Self {
        NbtEdit {
            screen: Screen::Welcome,
            directory: None,
        }
    }

    fn title(&self) -> String {
        format!("nbtedit {}", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::ChangeScreen(s) => self.screen = s,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .push(Row::new()
            .push(Row::new()
                .push(self.screen_btn(Screen::Open))
                .push(self.screen_btn(Screen::Save))
                .push(self.screen_btn(Screen::Apply))
            ).push(Row::new()
                .push(self.screen_btn(Screen::Level))
                .push(self.screen_btn(Screen::Player))
                .push(self.screen_btn(Screen::Dim(0)))
            ).push(Space::new(Length::Fill, Length::Shrink))
            .push(Row::new()
                .push(self.screen_btn(Screen::Settings))
                .push(self.screen_btn(Screen::Help))
            ).spacing(4)
            .padding(4)
        ).push(Container::new(match self.screen {
            Screen::Welcome => self.welcome(),
            Screen::Open => "open".into(),
            Screen::Save => "save".into(),
            Screen::Apply => "A".into(),
            Screen::Level => "lv".into(),
            Screen::Player => "play".into(),
            Screen::Dim(_) => "d".into(),
            Screen::Settings => "set".into(),
            _ => "a".into()
        }).height(Length::Fill))
        .align_items(Alignment::Center)
        .into()
    }
}

fn main() -> iced::Result {
    NbtEdit::run(Settings {
        window: window::Settings {
            size: (800, 600),
            min_size: Some((800, 600)),
            platform_specific: PlatformSpecific {
                application_id: format!("{}", "CARGO_PKG_NAME")
            },
            ..Default::default()
        },
        ..Default::default()
    })
}
