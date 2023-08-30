mod helpers;
mod nbt;

use helpers::{btn_centered, default_paths, dir_buttons, labeled_element};
use iced::{Sandbox, Settings, window::{self, PlatformSpecific}, widget::{Button, Space, Row, Column, Container, Scrollable, TextInput}, Length, Alignment};
use nbt::{NbtFile, TagMessage, TagType};

struct NbtEdit {
    screen: Screen,
    selected_tag: Option<TagType>,
    selected_name: String,
    selected_id: Option<usize>,
    temp_value: String,
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
        let screen = if let Some(t) = self.selected_tag.as_ref() {
            match t {
                TagType::End => self.screen_blank(),
                TagType::Byte(_) |
                TagType::Short(_) |
                TagType::Int(_) |
                TagType::Long(_) => self.screen_number(),
                TagType::Float(_) => self.screen_float(),
                TagType::Double(_) => self.screen_double(),
                TagType::ByteArray(_) => todo!(),
                TagType::String(_) => self.screen_string(),
                TagType::List(_) => todo!(),
                TagType::Compound(_) => self.screen_blank(),
                TagType::IntArray(_) => todo!(),
                TagType::LongArray(_) => todo!(),
            }
        } else {
            Column::new()
        };

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
    
    pub fn screen_blank(&self) -> Column<'_, AppMessage> {
        let mut column = Column::new().width(Length::FillPortion(2));
    
        if let Some(_) = &self.selected_id {
            column = column.push(labeled_element(
                "key",
                TextInput::new("", &self.selected_name)
                    .on_input(|x| AppMessage::TagEvent(self.selected_id.unwrap(), crate::nbt::TagMessage::EditKey(x))).into()
            ));
        } else {
            column = column.push("no value selected");
        }
    
        column
    }
    
    pub fn screen_number(&self) -> Column<'_, AppMessage> {
        Column::new().width(Length::FillPortion(2))
            .push(labeled_element(
                "key",
                TextInput::new("", &self.selected_name)
                    .on_input(|x| AppMessage::TagEvent(self.selected_id.unwrap(), TagMessage::EditKey(x))).into()
            ))
            .push(labeled_element(
                "value",
                TextInput::new("", &self.temp_value)
                    .on_input(AppMessage::InputValue)
                    .on_submit(if let Ok(x) = self.temp_value.parse::<i64>() {
                            AppMessage::TagEvent(self.selected_id.unwrap(), TagMessage::EditTag(
                            match self.selected_tag.as_ref().unwrap() {
                                TagType::Byte(_) => TagType::Byte(x as i8),
                                TagType::Short(_) => TagType::Short(x as i16),
                                TagType::Int(_) => TagType::Int(x as i32),
                                TagType::Long(_) => TagType::Long(x),
                                _ => unreachable!("not accessible from other tags"),
                            }
                        ))} else {
                            AppMessage::Pass
                        })
                    .into()
            ))
    }

    pub fn screen_float(&self) -> Column<'_, AppMessage> {
        Column::new().width(Length::FillPortion(2))
            .push(labeled_element(
                "key",
                TextInput::new("", &self.selected_name)
                    .on_input(|x| AppMessage::TagEvent(self.selected_id.unwrap(), TagMessage::EditKey(x))).into()
            ))
            .push(labeled_element(
                "value",
                TextInput::new("", &self.temp_value)
                    .on_input(AppMessage::InputValue)
                    .on_submit(if let Ok(x) = self.temp_value.parse() {
                            AppMessage::TagEvent(self.selected_id.unwrap(), TagMessage::EditTag(
                            match self.selected_tag.as_ref().unwrap() {
                                TagType::Float(_) => TagType::Float(x),
                                _ => unreachable!("not accessible from other tags"),
                            }
                        ))} else {
                            AppMessage::Pass
                        })
                    .into()
            ))
    }

    pub fn screen_double(&self) -> Column<'_, AppMessage> {
        Column::new().width(Length::FillPortion(2))
            .push(labeled_element(
                "key",
                TextInput::new("", &self.selected_name)
                    .on_input(|x| AppMessage::TagEvent(self.selected_id.unwrap(), TagMessage::EditKey(x))).into()
            ))
            .push(labeled_element(
                "value",
                TextInput::new("", &self.temp_value)
                    .on_input(AppMessage::InputValue)
                    .on_submit(if let Ok(x) = self.temp_value.parse() {
                            AppMessage::TagEvent(self.selected_id.unwrap(), TagMessage::EditTag(
                            match self.selected_tag.as_ref().unwrap() {
                                TagType::Double(_) => TagType::Double(x),
                                _ => unreachable!("not accessible from other tags"),
                            }
                        ))} else {
                            AppMessage::Pass
                        })
                    .into()
            ))
    }

    pub fn screen_string(&self) -> Column<'_, AppMessage> {
        Column::new().width(Length::FillPortion(2))
            .push(labeled_element(
                "key",
                TextInput::new("", &self.selected_name)
                    .on_input(|x| AppMessage::TagEvent(self.selected_id.unwrap(), TagMessage::EditKey(x))).into()
            ))
            .push(labeled_element(
                "value",
                TextInput::new("", &self.temp_value)
                    .on_input(AppMessage::InputValue)
                    .on_submit(AppMessage::TagEvent(self.selected_id.unwrap(), TagMessage::EditTag(TagType::String(self.temp_value.clone()))))
                    .into()
            ))
    }
}

impl Sandbox for NbtEdit {
    type Message = AppMessage;

    fn new() -> Self {
        NbtEdit {
            screen: Screen::Welcome,
            selected_tag: None,
            selected_name: String::new(),
            selected_id: None,
            temp_value: String::new(),
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
            AppMessage::TagEvent(id, TagMessage::SelectTag(name, t)) => {
                self.selected_name = name;
                self.selected_id = Some(id);
                
                match &t {
                    TagType::Byte(v) => self.temp_value = v.to_string(),
                    TagType::Short(v) => self.temp_value = v.to_string(),
                    TagType::Int(v) => self.temp_value = v.to_string(),
                    TagType::Long(v) => self.temp_value = v.to_string(),
                    TagType::Float(v) => self.temp_value = v.to_string(),
                    TagType::Double(v) => self.temp_value = v.to_string(),
                    TagType::String(v) => self.temp_value = v.to_owned(),
                    _ => {},
                }

                self.selected_tag = Some(t);
            },
            AppMessage::TagEvent(id, TagMessage::RemoveTag) => {},
            AppMessage::TagEvent(id, TagMessage::EditKey(key)) => {
                self.selected_name = key.clone();
                self.level_dat.as_mut().unwrap().get_mut_tag().find(id).unwrap().update(TagMessage::EditKey(key));
            }
            AppMessage::TagEvent(id, TagMessage::EditTag(t)) => {
                match &t {
                    TagType::Byte(v) => self.temp_value = v.to_string(),
                    TagType::Short(v) => self.temp_value = v.to_string(),
                    TagType::Int(v) => self.temp_value = v.to_string(),
                    TagType::Long(v) => self.temp_value = v.to_string(),
                    TagType::Float(v) => self.temp_value = v.to_string(),
                    TagType::Double(v) => self.temp_value = v.to_string(),
                    TagType::String(v) => self.temp_value = v.to_owned(),
                    _ => {},
                }
            }
            AppMessage::TagEvent(id, msg) => self.level_dat.as_mut().unwrap().get_mut_tag().find(id).unwrap().update(msg),
            AppMessage::InputValue(value) => self.temp_value = value,
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
