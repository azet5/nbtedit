mod helpers;
mod nbt;
mod queue;

use helpers::{btn_centered, dir_buttons, is_mc_save, labeled_element, TagChoice};
use iced::{Sandbox, Settings, window::{self, PlatformSpecific}, widget::{Button, Space, Row, Column, Container, Scrollable, TextInput, Text, PickList}, Length, Alignment};
use nbt::{NbtFile, Tag, TagMessage, TagType};
use queue::{ActionQueue, ActionType};

struct NbtEdit {
    screen: Screen,
    create_screen: ToggleCreateMode,
    create_data: Option<CreateData>,
    // selected_tag: Option<TagType>,
    // selected_name: Option<String>,
    // selected_id: Option<usize>,
    // selected_value: Option<String>,
    selected_tag: Option<Tag>,
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

impl Screen {
    fn create_btn<'a>(&self) -> Button<'a, AppMessage> {
        let text = match self {
            Screen::Welcome => unreachable!("this button does not exist"),
            Screen::Open => "Open",
            Screen::Save => "Save",
            Screen::Edit => "Edit",
            Screen::Settings => "Settings",
            Screen::Help => "Help",
        };
        btn_centered(text, Length::Fixed(70.0)).on_press(AppMessage::ChangeScreen(self.clone()))
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    ChangeScreen(Screen),
    CheckPath(String),
    TagEvent(usize, TagMessage),
    InputKey(String),
    InputValue(String),
    ToggleCreate(ToggleCreateMode),
    InputTagName(String),
    InputTagValue(TagChoice),
    InputAction(ActionType),
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
                    options = options.push(labeled_element("type", Text::new(format!("{}", tag.get())).into()));
                },
            }
        }

        Column::new()
            .push(Text::new(format!("{} action(s)", self.queue.length())))
            .push(Row::new()
                .push(Scrollable::new(buttons).width(Length::FillPortion(2)))
                .push(options)
                .height(Length::Fill)
            ).push(Button::new("Apply"))
            .padding(4)
            .spacing(4)
            .into()
    }
        
    fn level(&self) -> iced::Element<'_, AppMessage> {
        let mut screen = Column::new().width(Length::FillPortion(2)).padding(4).spacing(4);
        match &self.create_screen {
            ToggleCreateMode::None => {
                if let Some(t) = self.selected_tag.as_ref() {
                    screen = screen.push(labeled_element("type", Text::new(t.get().type_name()).into()));
            
                    if let Some(name) = t.name() {
                        screen = screen.push(labeled_element(
                            "key",
                            TextInput::new("", name)
                                .on_input(AppMessage::InputKey).into()
                        ));
                    }
            
                    match t.get() {
                        TagType::End |
                        TagType::Compound(_) |
                        TagType::List(_) |
                        TagType::ByteArray(_) |
                        TagType::IntArray(_) |
                        TagType::LongArray(_) => {},
                        _ => screen = screen.push(labeled_element(
                            "value",
                            TextInput::new("", t.get().to_string().as_str())
                            .on_input(AppMessage::InputValue)
                            .into()
                        )),
                    }
            
                    screen = screen.push(Space::with_height(Length::Fill));
                    let mut row = Row::new()
                        .push(btn_centered("Apply", 100)
                            .on_press(AppMessage::TagEvent(t.id(), TagMessage::EditTag {
                                name: t.name().cloned(),
                                // value: Some(match t.get() {
                                //     TagType::Byte(_) => TagType::Byte(self.selected_value.as_ref().unwrap().parse().unwrap_or_default()),
                                //     TagType::Short(_) => TagType::Short(self.selected_value.as_ref().unwrap().parse().unwrap_or_default()),
                                //     TagType::Int(_) => TagType::Int(self.selected_value.as_ref().unwrap().parse().unwrap_or_default()),
                                //     TagType::Long(_) => TagType::Long(self.selected_value.as_ref().unwrap().parse().unwrap_or_default()),
                                //     TagType::Float(_) => TagType::Float(self.selected_value.as_ref().unwrap().parse().unwrap_or_default()),
                                //     TagType::Double(_) => TagType::Double(self.selected_value.as_ref().unwrap().parse().unwrap_or_default()),
                                //     TagType::String(_) => TagType::String(self.selected_value.as_ref().unwrap().parse().unwrap_or_default()),
                                //     _ => t.get().clone(),
                                // }),
                                value: Some(t.get().clone()),
                            }))
                        );
                    if let TagType::Compound(_) = t.get() {
                        row = row.push(btn_centered("Insert tag", 100).on_press(AppMessage::ToggleCreate(ToggleCreateMode::Tag)));
                    } else if let TagType::List(tags) = t.get() {
                        row = row.push(btn_centered("Insert", 100).on_press(if tags.len() > 0 {
                            AppMessage::ToggleCreate(ToggleCreateMode::ListItem(tags[0].get().clone()))
                        } else {
                            AppMessage::ToggleCreate(ToggleCreateMode::Tag)
                        }));
                    }
                    row = row.push(btn_centered("Delete", 100).on_press(AppMessage::TagEvent(t.id(), TagMessage::RemoveTag)));
                    screen = screen.push(row);
                }
            },
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
                    .push(btn_centered("Done", 70))
                    .push(btn_centered("Cancel", 70).on_press(AppMessage::ToggleCreate(ToggleCreateMode::None)));
            },
            ToggleCreateMode::ListItem(_) => {

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
                } else {
                    self.path = path;
                }
            },
            // AppMessage::ChangeOpenPath(p) => self.path = p,
            // AppMessage::OpenDirectory(d) => {
            //     let path = d.clone();
            //     self.directory = Some(d);
            //     self.selected_tag = None;
            //     self.screen = Screen::Edit;
            //     self.level_dat = Some(NbtFile::open(format!("{}/level.dat", path)).unwrap());
            // },
            AppMessage::TagEvent(id, TagMessage::SelectTag(name, t)) => {
                self.create_screen = ToggleCreateMode::None;
                self.selected_tag = Some(t);
            },
            AppMessage::TagEvent(id, TagMessage::RemoveTag) => {
                self.queue.add(ActionType::Delete(id));
                self.selected_tag = None;
                self.level_dat.as_mut().unwrap().get_mut_tag().find_mut(id).unwrap().update(TagMessage::RemoveTag);
            },
            // AppMessage::InputKey(key) => self.selected_name = Some(key),
            // AppMessage::TagEvent(id, ref e @ TagMessage::EditTag { .. }) => {
            //     self.queue.add(ActionType::Edit {
            //         id,
            //         old_name: self.selected_name.as_ref().unwrap().clone(),
            //         old_value: self.selected_tag.as_ref().unwrap().clone(),
            //     });
            //     self.level_dat.as_mut().unwrap().get_mut_tag().find_mut(id).unwrap().update(e.clone());
            // },
            AppMessage::TagEvent(id, msg) => self.level_dat.as_mut().unwrap().get_mut_tag().find_mut(id).unwrap().update(msg),
            // AppMessage::InputValue(value) => self.selected_value = Some(value),
            AppMessage::InputTagName(name) => self.create_data.as_mut().unwrap().name = name,
            AppMessage::InputTagValue(value) => self.create_data.as_mut().unwrap().tag = value,
            // self.create_data.as_mut().unwrap().tag = value,
            AppMessage::ToggleCreate(tag) => {
                match tag {
                    ToggleCreateMode::Tag => self.create_data = Some(Default::default()),
                    _ => self.create_data = None,
                }
                self.create_screen = tag;
            },
            AppMessage::InputAction(action) => self.selected_action = Some(action),
            AppMessage::Pass => {},
            _ => {},
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new()
        .push(Row::new()
            .push(Row::new()
                .push(Screen::Open.create_btn())
                .push(Screen::Save.create_btn())
                .push(Screen::Edit.create_btn()))
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(Row::new()
                .push(Screen::Settings.create_btn())
                .push(Screen::Help.create_btn())
            ).spacing(4)
            .padding(4)
        ).push(Container::new(match self.screen {
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
