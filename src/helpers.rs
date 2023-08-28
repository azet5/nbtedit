use std::{borrow::Cow, path::Path, ffi::OsString};

use iced::{widget::{Button, Text, Column, Row}, alignment::Horizontal, Length, Padding};

use crate::{AppMessage, nbt::{NbtFile, TagType, Tag}};

#[macro_export]
macro_rules! is_dir {
    ($self: expr, $t: expr) => {
        if let Some(_) = $self.directory {
            Some($t)
        } else {
            None
        }
    };
}

pub fn btn_centered<'a>(text: impl Into<Cow<'a, str>>, width: impl Into<Length>) -> Button<'a, AppMessage> {
    Button::new(Text::new(text).horizontal_alignment(Horizontal::Center)).padding(4).width(width)
}

fn btn_to_path<'a>(path: impl Into<String>, label: impl Into<Cow<'a, str>>) -> Button<'a, AppMessage> {
    btn_centered(label, 100).on_press(AppMessage::ChangeOpenPath(path.into()))
}

fn btn_to_save<'a>(path: impl Into<String>, label: impl Into<Cow<'a, str>>) -> Button<'a, AppMessage> {
    Button::new(Text::new(label)).on_press(AppMessage::OpenDirectory(path.into()))
}

pub fn default_paths<'a>() -> Column<'a, AppMessage> {
    #[cfg(target_family = "unix")]
    {
        Column::new()
            .push(btn_to_path(std::env::var("HOME").unwrap_or("~/".to_string()), "home"))
            .push(btn_to_path(if let Ok(s) = std::env::var("HOME") {
                format!("{}/.local/share", s)
            } else {
                "~/.local/share".to_string()
            }, "share"))
            .padding(4).spacing(4)
    }
    #[cfg(target_os = "windows")]
    {
        Column::new()
        .push(btn_to_path("%userprofile%", "home"))
        .push(btn_to_path("%appdata%", "share"))
    }
}

// TODO: more reliable check
fn is_mc_save(path: &Path) -> bool {
    if let Ok(mut dir) = std::fs::read_dir(path) {
        dir.find(|p| p.as_ref().unwrap().file_name() == OsString::from("level.dat")).is_some()
    } else {
        false
    }
}

fn list_dir(path: &Path) -> Result<Vec<(String, String)>, String> {
    match std::fs::read_dir(path) {
        Ok(dir) => {
            let mut data = Vec::new();
            for i in dir {
                match i {
                    Ok(dir) => {
                        if dir.file_type().unwrap().is_dir() {
                            data.push((dir.file_name().to_str().unwrap().to_string(), dir.path().to_str().unwrap().to_string()));
                        }
                    },
                    Err(e) => return Err(e.to_string()),
                }
            }

            data.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

            Ok(data)
        },
        Err(e) => Err(e.to_string()),
    }
}

pub fn dir_buttons<'a>(path: impl Into<Cow<'a, str>> + std::convert::AsRef<std::path::Path>) -> Column<'a, AppMessage> {
    let mut list = Column::new().padding(4).spacing(4);
    if path.as_ref() != Path::new("/") {
        list = list.push(btn_to_path(path.as_ref().parent().unwrap().to_str().unwrap(), "..").width(Length::Fill))
    }
    match list_dir(path.as_ref()) {
        Ok(data) => {
            for entry in data {
                if is_mc_save(&Path::new(&entry.1)) {
                    list = list.push(btn_to_save(entry.1, entry.0).width(Length::Fill).style(iced::theme::Button::Positive));
                } else {
                    list = list.push(btn_to_path(entry.1, entry.0).width(Length::Fill));
                }
            }
        },
        Err(_) => {
            list = list.push("Cannot open");
        }
    }

    return list;
}

fn tree_btn<'a>(name: impl Into<&'a str>) -> Row<'a, AppMessage> {
    Row::new()
        // .push(Button::new("+"))
        .push(Button::new(name.into()))
}

fn tree_child<'a>(tag: &'a Tag) -> Column<'a, AppMessage> {
    let mut column = Column::new().padding([0, 0, 0, 16]);
    match tag.get_tag() {
        TagType::Compound(tags) => {
            let name = tag.get_name().as_deref();
            column = column.push(tree_btn(name.unwrap_or("(empty)")));
            for t in tags {
                column = column.push(tree_child(t));
            }
        },
        t => {
            let name = tag.get_name().as_deref();
            column = column.push(tree_btn(name.unwrap_or("(empty)")));
        },
    }

    column
}

pub fn nbt_tree<'a>(data: &'a NbtFile) -> Column<'a, AppMessage> {
    Column::new().push(tree_child(data.get_tag()))
}
