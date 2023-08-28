mod helpers;
mod nbt;
mod widgets;

use helpers::{btn_centered, default_paths, dir_buttons, nbt_tree};
use iced::{Sandbox, Settings, window::{self, PlatformSpecific}, widget::{Button, Space, Row, Column, Container, Scrollable}, Length, Alignment};
use nbt::NbtFile;

struct NbtEdit {
    screen: Screen,
    path: String,
    directory: Option<String>,
    level_dat: Option<NbtFile>,
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
    ChangeOpenPath(String),
    OpenDirectory(String),
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

    fn open(&self) -> iced::Element<'_, AppMessage> {
        Column::new()
            .push(Row::new()
                .push(Scrollable::new(default_paths()))
                .push(Scrollable::new(dir_buttons(&self.path)).width(Length::Fill))
            )
            .height(Length::Fill)
            .spacing(4)
            .padding(4)
            .align_items(Alignment::Center)
            .into()
    }

    fn save(&self) -> iced::Element<'_, AppMessage> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push("Save")
            .push(Space::new(Length::Fill, Length::Fill))
            .align_items(Alignment::Center)
            .into()
    }

    fn apply(&self) -> iced::Element<'_, AppMessage> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push("Apply")
            .push(Space::new(Length::Fill, Length::Fill))
            .align_items(Alignment::Center)
            .into()
    }

    fn level(&self) -> iced::Element<'_, AppMessage> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push(Scrollable::new(Column::new().push(nbt_tree(self.level_dat.as_ref().unwrap()))))
            .push(Space::new(Length::Fill, Length::Fill))
            .align_items(Alignment::Center)
            .into()
    }

    fn player(&self) -> iced::Element<'_, AppMessage> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push("player.dat")
            .push(Space::new(Length::Fill, Length::Fill))
            .align_items(Alignment::Center)
            .into()
    }

    fn dim(&self) -> iced::Element<'_, AppMessage> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push("dim")
            .push(Space::new(Length::Fill, Length::Fill))
            .align_items(Alignment::Center)
            .into()
    }

    fn settings(&self) -> iced::Element<'_, AppMessage> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push("Settings")
            .push(Space::new(Length::Fill, Length::Fill))
            .align_items(Alignment::Center)
            .into()
    }

    fn help(&self) -> iced::Element<'_, AppMessage> {
        Column::new()
            .push(Space::new(Length::Fill, Length::Fill))
            .push("Help")
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
            path: "/home".to_string(),
            directory: None,
            level_dat: None,
        }
    }

    fn title(&self) -> String {
        format!("nbtedit {}", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::ChangeScreen(s) => self.screen = s,
            AppMessage::ChangeOpenPath(p) => self.path = p,
            AppMessage::OpenDirectory(d) => {
                let path = d.clone();
                self.directory = Some(d);
                self.screen = Screen::Level;
                self.level_dat = Some(NbtFile::open(format!("{}/level.dat", path)).unwrap());
            },
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
            Screen::Open => self.open(),
            Screen::Save => self.save(),
            Screen::Apply => self.apply(),
            Screen::Level => self.level(),
            Screen::Player => self.player(),
            Screen::Dim(_) => self.dim(),
            Screen::Settings => self.settings(),
            Screen::Help => self.help(),
            _ => todo!("unreachable by now"),
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
