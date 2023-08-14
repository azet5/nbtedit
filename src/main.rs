mod helpers;

use iced::{Sandbox, Settings, window::{self, PlatformSpecific}, widget::{Button, Space, Rule, Row, TextInput, Column, Container}, Length};

struct NbtEdit {
    screen: Screen,
    directory: Option<String>
}

#[derive(Debug, Clone)]
enum Screen {
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
enum AppMessage {
    ChangeScreen(Screen),
}

impl NbtEdit {
    fn screen_btn(&self, screen: Screen) -> Button<'_, AppMessage> {
        match screen {
            Screen::Open => Button::new("Open").width(60).on_press(AppMessage::ChangeScreen(Screen::Open)),
            Screen::Save => Button::new("Save").width(60).on_press(AppMessage::ChangeScreen(Screen::Save)),
            Screen::Apply => Button::new("Apply").width(60).on_press(AppMessage::ChangeScreen(Screen::Apply)),
            Screen::Level => Button::new("level.dat").width(90).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Level))),
            Screen::Player => Button::new("player.dat").width(90).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Player))),
            Screen::Dim(_) => Button::new("DIM*.dat").width(90).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Dim(0)))),
            Screen::Settings => Button::new("Settings").width(90).on_press(AppMessage::ChangeScreen(Screen::Settings)),
            Screen::Help => Button::new("?").width(20).on_press(AppMessage::ChangeScreen(Screen::Help)),
            _ => unreachable!("no buttons for other types exist"),
        }
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
            )
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(self.screen_btn(Screen::Settings))
            .spacing(8).padding(8).height(Length::Shrink)
        ).push(Container::new(match self.screen {
            Screen::Welcome => "Welcome!",
            Screen::Open => "open",
            Screen::Save => "save",
            Screen::Apply => "A",
            Screen::Level => "lv",
            Screen::Player => "play",
            Screen::Dim(_) => "d",
            Screen::Settings => "set",
            _ => "a"
        }).height(Length::Fill)).into()
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
