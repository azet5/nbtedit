use std::{borrow::Cow, path::Path, fs::DirEntry, ffi::OsString};

use iced::{widget::{Button, Text, Column}, alignment::Horizontal, Length};

use crate::AppMessage;

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

// obscure workaround which doesn't make temporary value,
// because i don't know rust
// TODO: delete this
fn get_owned_name(entry: DirEntry) -> (String, DirEntry) {
    (entry.file_name().to_str().unwrap().to_owned(), entry)
}

pub fn list_dir<'a>(path: impl Into<Cow<'a, str>> + std::convert::AsRef<std::path::Path>) -> Column<'a, AppMessage> {
    let mut list = Column::new().padding(4).spacing(4);
    if path.as_ref() != Path::new("/") {
        list = list.push(btn_to_path(path.as_ref().parent().unwrap().to_str().unwrap(), "..").width(Length::Fill))
    }

    match std::fs::read_dir(path) {
        Ok(t) => {
            for i in t {
                if let Ok(s) = i {
                    if s.file_type().unwrap().is_dir() {
                        let path = s.path();
                        if is_mc_save(&s.path()) {
                            list = list.push(Button::new(Text::new(get_owned_name(s).0)).on_press(AppMessage::OpenDirectory(path.to_str().unwrap().to_string())).width(Length::Fill).style(iced::theme::Button::Positive));
                        } else {
                            list = list.push(btn_to_path(path.to_str().unwrap(), get_owned_name(s).0).width(Length::Fill));
                        }
                    }
                }
            }
        },
        Err(_) => {
            list = list.push("cannot open");
        },
    }

    return list;
}