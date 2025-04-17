mod helpers;
mod nbt;
mod queue;

use helpers::{btn_centered, dir_buttons, is_mc_save, labeled_element, TagChoice};
use iced::{widget::{Button, Column, Container, PickList, Row, Rule, Scrollable, Space, Text, TextInput}, window::{self, settings::PlatformSpecific}, Alignment, Length, Sandbox, Settings, Size};
use nbt::{NbtFile, Tag, TagMessage, TagType};
use queue::{ActionQueue, ActionType};

struct NbtEdit {
    screen: Screen,
    create_screen: ToggleCreateMode,
    create_data: Option<CreateData>,
    selected_tag: Option<Tag>,
    temp_name: String,
    temp_value: String,
    path: String,
    level_dat: Option<NbtFile>,
    queue: ActionQueue,
    selected_action: Option<ActionType>,
}

#[derive(Debug, Clone)]
pub enum Screen {
    Welcome,
    Open,
    Save,
    Edit,
    Settings,
    Help,
}

impl NbtEdit {
    fn create_btn<'a>(&self, screen: Screen) -> Button<'a, AppMessage> {
        let text = match screen {
            Screen::Welcome => unreachable!("this button does not exist"),
            Screen::Open => "Open",
            Screen::Save => "Save",
            Screen::Edit => "Edit",
            Screen::Settings => "Settings",
            Screen::Help => "Help",
        };

        match screen {
            Screen::Edit => btn_centered(text, Length::Fixed(70.0))
                .on_press_maybe(if self.level_dat.is_some() {
                    Some(AppMessage::ChangeScreen(screen))
                } else { None }),
            _ => btn_centered(text, Length::Fixed(70.0)).on_press(AppMessage::ChangeScreen(screen))
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    ChangeScreen(Screen),
    CheckPath(String),
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

impl NbtEdit {
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
                // .push(Scrollable::new(default_paths()))
                .push(Scrollable::new(dir_buttons(&self.path)).width(Length::Fill))
            )
            .height(Length::Fill)
            .spacing(4)
            .padding(4)
            .align_items(Alignment::Center)
            .into()
    }

    fn save(&self) -> iced::Element<'_, AppMessage> {
        let mut buttons = Column::new().padding(4).spacing(4);
        let mut options = Column::new().width(Length::FillPortion(2)).padding(4).spacing(4);
        
        if self.queue.length() > 0 {
            for item in self.queue.iter() {
                let text = match item {
                    ActionType::Add { id, .. } => format!(
                        "add: {}",
                        self.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap().name().unwrap_or(&"(empty)".to_string())
                    ),
                    ActionType::Delete(id) => format!(
                        "delete: {}",
                        self.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap().name().unwrap_or(&"(empty)".to_string())
                    ),
                    ActionType::Edit { id, .. } => format!(
                        "edit: {}",
                        self.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap().name().unwrap_or(&"(empty)".to_string())
                    ),
                };
                
                buttons = buttons.push(btn_centered(text, Length::Fill).on_press(AppMessage::InputAction(item.clone())));
            }
        }
        
        if let Some(action) = &self.selected_action {
            match action {
                ActionType::Add { id, .. } => {
                    let tag = self.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap();
                    options = options.push(labeled_element("type", Text::new(format!("{}", tag.get())).into()));
                },
                ActionType::Edit { id, old_name, .. } => {
                    let tag = self.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap();
                    options = options.push(labeled_element("type", Text::new(format!("{}", tag.get())).into()));
                    options = options.push(labeled_element("old key", Text::new(old_name).into()));
                }
                ActionType::Delete(id) => {
                    let tag = self.level_dat.as_ref().unwrap().get_tag().find(*id).unwrap();
                    options = options.push(labeled_element("Type:", Text::new(tag.get().type_name()).into()));
                    options = options.push(labeled_element("Value:", Text::new(tag.get().to_string()).into()));
                },
            }
        }

        Column::new()
            .push(Row::new()
                .push(Button::new("Apply").on_press_maybe(if self.queue.length() > 0 {
                    Some(AppMessage::Write)
                } else { None }))
                .push(Text::new(format!("{} action(s)", self.queue.length())))
                .padding(4)
                .spacing(4)
                .align_items(Alignment::Center)
            ).push(Row::new()
                .push(Scrollable::new(buttons).width(Length::FillPortion(2)))
                .push(options)
                .height(Length::Fill)
            ).padding(4)
            .spacing(4)
            .into()
    }
        
    fn level(&self) -> iced::Element<'_, AppMessage> {
        let mut screen = Column::new().width(Length::FillPortion(2)).padding(8).spacing(8);
        if let Some(t) = self.selected_tag.as_ref() {
            screen = screen.push(labeled_element("Tag Type:", Text::new(t.get().type_name()).into()));
    
            if let Some(_) = t.name() {
                screen = screen.push(labeled_element(
                    "Key",
                    TextInput::new("", &self.temp_name)
                        // .on_input(AppMessage::InputName)
                        .into()
                ));
            }
    
            let content = t.get().to_string();
            if t.get().is_compound() {
                screen = screen.push(labeled_element("Content:", Text::new(content).into()));
            } else {
                screen = screen.push(labeled_element(
                    "Value",
                    TextInput::new("123", &self.temp_value)
                        .on_input(AppMessage::InputValue).into()
                ));
            }
    
            screen = screen.push(Space::with_height(20));
            let mut row = Row::new().spacing(4)
                .push(btn_centered("Apply", 100)
                    .on_press_maybe(if t.get().is_compound() {
                            None
                        } else {
                            Some(AppMessage::TagEvent(t.id(), TagMessage::EditTag {
                                name: t.name().cloned(),
                                value: Some(t.get().replace(&self.temp_value)),
                            }))
                        })
                    // .on_press_maybe(None)
                );
            if let TagType::Compound(_) = t.get() {
                row = row.push(btn_centered("Insert Tag", 100).on_press(AppMessage::ToggleCreate(ToggleCreateMode::Tag)));
            } else if let TagType::List(tags) = t.get() {
                row = row.push(btn_centered("Insert Item", 100).on_press(if tags.len() > 0 {
                    AppMessage::ToggleCreate(ToggleCreateMode::ListItem(tags[0].get().clone()))
                } else {
                    AppMessage::ToggleCreate(ToggleCreateMode::Tag)
                }));
            }
            row = row.push(btn_centered("Delete", 100).on_press(AppMessage::TagEvent(t.id(), TagMessage::RemoveTag)));
            screen = screen.push(row);
        }

        match &self.create_screen {
            ToggleCreateMode::None => {},
            ToggleCreateMode::Tag => {
                screen = screen.push("Insert new tag")
                    .push(labeled_element("Name", TextInput::new(
                        "",
                        self.create_data.as_ref().unwrap().name.as_str()
                    ).on_input(|x| AppMessage::InputTagName(x)).into()))
                    .push(labeled_element("Type", PickList::new(
                        &TagChoice::ALL[..],
                        Some(self.create_data.as_ref().unwrap().tag),
                        |x| AppMessage::InputTagValue(x)).into()))
                    .push(Row::new().spacing(4)
                        .push(btn_centered("Done", 70))
                        .push(btn_centered("Cancel", 70).on_press(AppMessage::ToggleCreate(ToggleCreateMode::None)))   
                    );
            },
            ToggleCreateMode::ListItem(_) => {
                screen = screen.push("Insert new item")
                    .push(labeled_element("Value", TextInput::new(
                        "",
                        self.create_data.as_ref().unwrap().name.as_str()
                    ).into()))
                    .push(Row::new().spacing(4)
                        .push(btn_centered("Done", 70))
                        .push(btn_centered("Cancel", 70).on_press(AppMessage::ToggleCreate(ToggleCreateMode::None)))   
                    );
            },
        }

        Row::new()
            .push(Scrollable::new(self.level_dat.as_ref().unwrap().get_tag().view()).width(Length::FillPortion(2)))
            .push(screen)
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
            create_screen: ToggleCreateMode::None,
            create_data: None,
            selected_tag: None,
            temp_name: String::new(),
            temp_value: String::new(),
            #[cfg(target_family = "unix")]
            path: "/home".to_string(),
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
            AppMessage::ChangeScreen(s) => self.screen = s,
            AppMessage::CheckPath(path) => {
                if is_mc_save(&path) {
                    self.screen = Screen::Edit;
                    self.level_dat = Some(NbtFile::open(format!("{}/level.dat", path)).unwrap());
                    self.path = format!("{}/level.dat", path);
                } else {
                    self.path = path;
                }
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
                self.level_dat.as_mut().unwrap().write(self.path.clone()).unwrap();
            },
            AppMessage::Pass => {},
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .push(Row::new()
            .push(Row::new()
                .push(self.create_btn(Screen::Open))
                .push(self.create_btn(Screen::Save))
                .push(self.create_btn(Screen::Edit))
                .spacing(2)
            ).push(Space::new(Length::Fill, Length::Shrink))
            .push(Row::new()
                .push(self.create_btn(Screen::Settings))
                .push(self.create_btn(Screen::Help))
                .spacing(2)
            ).padding(4)
        ).push(Rule::horizontal(1))
        .push(Container::new(match self.screen {
            Screen::Welcome => self.welcome(),
            Screen::Open => self.open(),
            Screen::Save => self.save(),
            Screen::Edit => self.level(),
            Screen::Settings => self.settings(),
            Screen::Help => self.help(),
        }).height(Length::Fill))
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
