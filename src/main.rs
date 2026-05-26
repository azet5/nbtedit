mod helpers;
mod nbt;
mod queue;
pub mod screen;

use helpers::{btn_centered, is_mc_save, TagChoice};
use iced::{widget::{Button, Column, Container, Row, Rule, Space}, window::{self, settings::PlatformSpecific}, Alignment, Length, Sandbox, Settings, Size};
use nbt::{NbtFile, Tag, TagMessage, TagType};
use queue::{ActionQueue, ActionType};

use crate::{helpers::CurrentPath, screen::ScreenTy};

pub struct NbtEdit {
    screen: Box<dyn screen::Screen>,
    create_screen: ToggleCreateMode,
    create_data: Option<CreateData>,
    selected_tag: Option<Tag>,
    temp_name: String,
    temp_value: String,
    path: CurrentPath,
    level_dat: Option<NbtFile>,
    queue: ActionQueue,
    selected_action: Option<ActionType>,
}

impl NbtEdit {
    fn create_btn<'a>(&self, screen: ScreenTy) -> Button<'a, AppMessage> {
        let text = match screen {
            ScreenTy::Welcome => unreachable!("this button does not exist"),
            ScreenTy::Open => "Open",
            ScreenTy::Save => "Save",
            ScreenTy::Edit => "Edit",
            ScreenTy::Settings => "Settings",
            ScreenTy::Help => "Help",
        };

        match screen {
            ScreenTy::Edit => btn_centered(text, Length::Fixed(70.0))
                .on_press_maybe(if self.level_dat.is_some() && self.screen.get_type() != screen {
                    Some(AppMessage::ChangeScreen(screen))
                } else { None }),
            _ => btn_centered(text, Length::Fixed(70.0)).on_press_maybe(
                if self.screen.get_type() != screen {
                    Some(AppMessage::ChangeScreen(screen))
                } else { None }
            )
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    ChangeScreen(ScreenTy),
    ChangeDir(String),
    TryOpenFile(String),
    TagEvent(usize, TagMessage),
    InputName(String),
    InputValue(String),
    ToggleCreate(ToggleCreateMode),
    InputTagName(String),
    InputTagValue(TagChoice),
    InputAction(ActionType),
    Write,
    Pass,
}

#[derive(Debug, Clone)]
pub enum ToggleCreateMode {
    None,
    Tag,
    ListItem(TagType),
}

#[derive(Debug, Clone)]
pub struct CreateData {
    name: String,
    tag: TagChoice,
}

impl Default for CreateData {
    fn default() -> Self {
        Self {
            name: String::new(),
            tag: TagChoice::End,
        }
    }
}

impl Sandbox for NbtEdit {
    type Message = AppMessage;

    fn new() -> Self {
        NbtEdit {
            screen: Box::new(screen::welcome::WelcomeScreen),
            create_screen: ToggleCreateMode::None,
            create_data: None,
            selected_tag: None,
            temp_name: String::new(),
            temp_value: String::new(),
            #[cfg(target_family = "unix")]
            path: CurrentPath::new("/home").unwrap(),
            #[cfg(target_os = "windows")]
            path: "C:/Users".to_string(),
            level_dat: None,
            queue: ActionQueue::new(),
            selected_action: None,
        }
    }

    fn title(&self) -> String {
        format!("nbtedit {}", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::ChangeScreen(s) => {
                self.screen = s.get_screen();
            },
            AppMessage::ChangeDir(path) => {
                self.path = CurrentPath::new(path).unwrap();
            },
            AppMessage::TryOpenFile(path) => {
                if is_mc_save(&path) {
                    self.screen = ScreenTy::Edit.get_screen();
                    self.level_dat = Some(NbtFile::open(&path).unwrap());
                }

                self.path = CurrentPath::new(path).unwrap();
            },
            AppMessage::TagEvent(_, TagMessage::SelectTag(_, t)) => {
                self.temp_name = t.name().unwrap_or(&String::new()).to_string();
                self.temp_value = t.get().to_string();
                self.create_screen = ToggleCreateMode::None;
                self.selected_tag = Some(t);
            },
            AppMessage::TagEvent(id, TagMessage::RemoveTag) => {
                self.queue.add(ActionType::Delete(id));
                self.selected_tag = None;
                self.level_dat.as_mut().unwrap().get_mut_tag().find_mut(id).unwrap().update(TagMessage::RemoveTag);
            },
            AppMessage::InputName(name) => self.temp_name = name,
            AppMessage::InputValue(value) => self.temp_value = value,
            AppMessage::TagEvent(id, ref e @ TagMessage::EditTag { .. }) => {
                self.queue.add(ActionType::Edit {
                    id,
                    old_name: self.selected_tag.as_ref().unwrap().name().as_ref().unwrap().to_string(),
                    old_value: self.selected_tag.as_ref().unwrap().get().clone(),
                });
                self.level_dat.as_mut().unwrap().get_mut_tag().find_mut(id).unwrap().update(e.clone());
            },
            AppMessage::TagEvent(id, msg) => self.level_dat.as_mut().unwrap().get_mut_tag().find_mut(id).unwrap().update(msg),
            AppMessage::InputTagName(name) => self.create_data.as_mut().unwrap().name = name,
            AppMessage::InputTagValue(value) => self.create_data.as_mut().unwrap().tag = value,
            AppMessage::ToggleCreate(tag) => {
                match tag {
                    ToggleCreateMode::None => self.create_data = None,
                    _ => self.create_data = Some(Default::default()),
                }
                self.create_screen = tag;
            },
            AppMessage::InputAction(action) => self.selected_action = Some(action),
            AppMessage::Write => {
                self.level_dat.as_mut().unwrap().write(self.path.get()).unwrap();
                self.queue.clear();
            },
            AppMessage::Pass => {},
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .push(Row::new()
            .push(Row::new()
                .push(self.create_btn(ScreenTy::Open))
                .push(self.create_btn(ScreenTy::Save))
                .push(self.create_btn(ScreenTy::Edit))
                .spacing(2)
            ).push(Space::new(Length::Fill, Length::Shrink))
            .push(Row::new()
                .push(self.create_btn(ScreenTy::Settings))
                .push(self.create_btn(ScreenTy::Help))
                .spacing(2)
            ).padding(4)
        ).push(Rule::horizontal(1))
        .push(Container::new(self.screen.view(&self)).height(Length::Fill))
        .align_items(Alignment::Center)
        .into()
    }
}

fn main() -> iced::Result {
    NbtEdit::run(Settings {
        window: window::Settings {
            size: Size::new(800.0, 600.0),
            min_size: Some(Size::new(800.0, 600.0)),
            #[cfg(target_os = "linux")]
            platform_specific: PlatformSpecific {
                application_id: format!("{}", "CARGO_PKG_NAME")
            },
            ..Default::default()
        },
        ..Default::default()
    })
}
