mod helpers;
mod nbt;
mod queue;
pub mod screen;
mod widget;
mod settings;

use helpers::{btn_centered, TagChoice};
use iced::{Alignment, Length, Size, widget::{Button, Column, Container, Row, Space, button, container, rule}, window::{self, settings::PlatformSpecific}};
use nbt::{NbtFile, Tag, TagMessage, TagType};
use queue::{ActionQueue, ActionType};

use crate::screen::ScreenTy;

pub struct NbtEdit {
    screen: Box<dyn screen::Screen>,
    create_screen: ToggleCreateMode,
    create_data: Option<CreateData>,
    selected_tag: Option<Tag>,
    temp_name: String,
    temp_value: String,
    dir_viewer: widget::DirViewer,
    settings: settings::AppSettings,
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

        btn_centered(text, Length::Fixed(70.0))
            .on_press(AppMessage::ChangeScreen(screen.clone())
            ).style(if self.screen.get_type() == screen {
                helpers::btn_active
            } else { button::subtle })
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    ChangeScreen(ScreenTy),
    // ChangeDir(String),
    TryOpenFile(String),
    TagEvent(usize, TagMessage),
    InputName(String),
    InputValue(String),
    ToggleCreate(ToggleCreateMode),
    InputTagName(String),
    InputTagValue(TagChoice),
    InputAction(ActionType),
    PassToBreadcrumb(widget::DirMessage),
    PassToSettings(settings::SettingsMessage),
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

impl NbtEdit {
    fn new() -> Self {
        NbtEdit {
            screen: Box::new(screen::welcome::WelcomeScreen),
            create_screen: ToggleCreateMode::None,
            create_data: None,
            selected_tag: None,
            temp_name: String::new(),
            temp_value: String::new(),
            #[cfg(target_family = "unix")]
            dir_viewer: widget::DirViewer::new("/home"),
            #[cfg(target_os = "windows")]
            dir_viewer: widget::DirViewer::new("C:/Users"),
            settings: settings::AppSettings::load(),
            level_dat: None,
            queue: ActionQueue::new(),
            selected_action: None,
        }
    }

    fn title(&self) -> String {
        format!("nbtedit {}", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::ChangeScreen(s) => {
                if self.screen.get_type() == ScreenTy::Settings && self.settings.is_dirty() {
                    self.settings.flush();
                }
                self.screen = s.get_screen();
            },
            AppMessage::TryOpenFile(path) => {
                self.screen = ScreenTy::Edit.get_screen();
                self.level_dat = Some(NbtFile::open(&path).unwrap());
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
            AppMessage::TagEvent(id, ref e @ TagMessage::EditTag { ref name, ref value }) => {
                self.queue.add(ActionType::Edit {
                    id,
                    old_name: self.selected_tag.as_ref().unwrap().name().as_ref().unwrap().to_string(),
                    old_value: self.selected_tag.as_ref().unwrap().get().clone(),
                    new_name: name.clone(),
                    new_value: value.clone(),
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
            AppMessage::PassToBreadcrumb(msg) => self.dir_viewer.update(msg),
            AppMessage::PassToSettings(msg) => self.settings.update(msg),
            AppMessage::Write => {
                // self.level_dat.as_mut().unwrap().write(self.path.get()).unwrap();
                self.queue.clear();
            },
            AppMessage::Pass => {},
        }
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        Column::new()
        .push(Row::new()
            .push(Container::new(
                Row::new()
                    .push(self.create_btn(ScreenTy::Open))
                    .push(self.create_btn(ScreenTy::Save))
                    .push(self.create_btn(ScreenTy::Edit))
                    .spacing(2)
                    .padding(2)
                ).style(container::rounded_box)
            ).push(Space::new().width(Length::Fill).height(Length::Shrink))
            .push(Container::new(
                Row::new()
                    .push(self.create_btn(ScreenTy::Settings))
                    .push(self.create_btn(ScreenTy::Help))
                    .spacing(2)
                    .padding(2)
                ).style(container::rounded_box)
            ).padding(4)
        ).push(rule::horizontal(1))
        .push(Container::new(self.screen.view(&self)).height(Length::Fill))
        .align_x(Alignment::Center)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(NbtEdit::new, NbtEdit::update, NbtEdit::view)
        .centered()
        .title(NbtEdit::title)
        .window(window::Settings {
            size: Size::new(800.0, 600.0),
            min_size: Some(Size::new(800.0, 600.0)),
            #[cfg(target_os = "linux")]
            platform_specific: PlatformSpecific {
                application_id: format!("{}", "CARGO_PKG_NAME"),
                ..Default::default()
            },
            ..Default::default()
        })
        .run()
}
