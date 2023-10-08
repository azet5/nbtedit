mod helpers;
mod nbt;

use helpers::{btn_centered, default_paths, dir_buttons, labeled_element};
use iced::{Sandbox, Settings, window::{self, PlatformSpecific}, widget::{Button, Space, Row, Column, Container, Scrollable, TextInput, Text}, Length, Alignment};
use nbt::{NbtFile, TagMessage, TagType};

struct NbtEdit {
    screen: Screen,
    selected_tag: Option<TagType>,
    selected_name: Option<String>,
    selected_id: Option<usize>,
    selected_value: Option<String>,
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
    Generic,
    Settings,
    Help,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    ChangeScreen(Screen),
    ChangeOpenPath(String),
    OpenDirectory(String),
    TagEvent(usize, TagMessage),
    InputKey(String),
    InputValue(String),
    Pass,
}

impl NbtEdit {
    fn screen_btn(&self, screen: Screen) -> Button<'_, AppMessage> {
        match screen {
            Screen::Open => btn_centered("Open", 60).on_press(AppMessage::ChangeScreen(Screen::Open)),
            Screen::Save => btn_centered("Save", 60),
            Screen::Apply => btn_centered("Apply", 60).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Apply))),
            Screen::Level => btn_centered("level.dat", 90).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Level))),
            Screen::Player => btn_centered("playerdata", 90).on_press_maybe(is_dir!(self, AppMessage::ChangeScreen(Screen::Player))),
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
        let mut screen = Column::new().width(Length::FillPortion(2));
        if let Some(t) = self.selected_tag.as_ref() {
            let str = format!("Type: {}", t.to_string());
            screen = screen.push(Text::new(str));

            if let Some(_) = &self.selected_name {
                screen = screen.push(labeled_element(
                    "key",
                    TextInput::new("", self.selected_name.as_ref().unwrap())
                        .on_input(AppMessage::InputKey).into()
                ));
                match t {
                    TagType::End |
                    TagType::Compound(_) |
                    TagType::List(_) |
                    TagType::ByteArray(_) |
                    TagType::IntArray(_) |
                    TagType::LongArray(_) => {},
                    _ => screen = screen.push(labeled_element(
                        "value",
                        TextInput::new("", self.selected_value.as_ref().unwrap())
                            .on_input(AppMessage::InputValue)
                            .into()
                    )),
                }
            }
        }

        Row::new()
            .push(Scrollable::new(self.level_dat.as_ref().unwrap().get_tag().view()).width(Length::FillPortion(2)))
            .push(screen)
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
            selected_tag: None,
            selected_name: None,
            selected_id: None,
            selected_value: None,
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
                self.selected_tag = None;
                self.screen = Screen::Level;
                self.level_dat = Some(NbtFile::open(format!("{}/level.dat", path)).unwrap());
            },
            AppMessage::TagEvent(id, TagMessage::SelectTag(name, t)) => {
                self.selected_name = name;
                self.selected_id = Some(id);
                
                match &t {
                    TagType::Byte(v) => self.selected_value = Some(v.to_string()),
                    TagType::Short(v) => self.selected_value = Some(v.to_string()),
                    TagType::Int(v) => self.selected_value = Some(v.to_string()),
                    TagType::Long(v) => self.selected_value = Some(v.to_string()),
                    TagType::Float(v) => self.selected_value = Some(v.to_string()),
                    TagType::Double(v) => self.selected_value = Some(v.to_string()),
                    TagType::String(v) => self.selected_value = Some(v.to_owned()),
                    _ => {},
                }

                self.selected_tag = Some(t);
            },
            AppMessage::TagEvent(id, TagMessage::RemoveTag) => self.level_dat.as_mut().unwrap().get_mut_tag().remove(id),
            AppMessage::InputKey(key) => self.selected_name = Some(key),
            AppMessage::TagEvent(id, TagMessage::EditTag { name, value }) => {

            },
            // AppMessage::TagEvent(id, TagMessage::EditKey(key)) => {
            //     self.selected_name = Some(key.clone());
            //     self.level_dat.as_mut().unwrap().get_mut_tag().find(id).unwrap().update(TagMessage::EditKey(key));
            // }
            // AppMessage::TagEvent(id, TagMessage::EditTag(t)) => {
            //     match &t {
            //         TagType::Byte(v) => self.selected_value = Some(v.to_string()),
            //         TagType::Short(v) => self.selected_value = Some(v.to_string()),
            //         TagType::Int(v) => self.selected_value = Some(v.to_string()),
            //         TagType::Long(v) => self.selected_value = Some(v.to_string()),
            //         TagType::Float(v) => self.selected_value = Some(v.to_string()),
            //         TagType::Double(v) => self.selected_value = Some(v.to_string()),
            //         TagType::String(v) => self.selected_value = Some(v.to_owned()),
            //         _ => {},
            //     }
            // }
            AppMessage::TagEvent(id, msg) => self.level_dat.as_mut().unwrap().get_mut_tag().find(id).unwrap().update(msg),
            AppMessage::InputValue(value) => self.selected_value = Some(value),
            AppMessage::Pass => {},
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
