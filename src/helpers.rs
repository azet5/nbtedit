use std::{borrow::Cow, fmt::{Display, Formatter}, fs::File, io::Read, path::Path, vec::IntoIter};

use flate2::read::GzDecoder;
use iced::{widget::{Button, Text, Column, Row}, alignment::Horizontal, Length, Element};

use crate::{AppMessage, nbt::{ParseError, ParserData}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagChoice {
    End,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl TagChoice {
    pub const ALL: [Self; 13] = [
        TagChoice::End,
        TagChoice::Byte,
        TagChoice::Short,
        TagChoice::Int,
        TagChoice::Long,
        TagChoice::Float,
        TagChoice::Double,
        TagChoice::ByteArray,
        TagChoice::String,
        TagChoice::List,
        TagChoice::Compound,
        TagChoice::IntArray,
        TagChoice::LongArray,
    ];
}

impl Display for TagChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TagChoice::End => "TAG_End",
            TagChoice::Byte => "TAG_Byte",
            TagChoice::Short => "TAG_Short",
            TagChoice::Int => "TAG_Int",
            TagChoice::Long => "TAG_Long",
            TagChoice::Float => "TAG_Float",
            TagChoice::Double => "TAG_Double",
            TagChoice::ByteArray => "TAG_ByteArray",
            TagChoice::String => "TAG_String",
            TagChoice::List => "TAG_List",
            TagChoice::Compound => "TAG_Compound",
            TagChoice::IntArray => "TAG_IntArray",
            TagChoice::LongArray => "TAG_LongArray",
        })
    }
}

pub struct CurrentPath {
    parts: Vec<(String, String)>,
    path: String,
}

impl CurrentPath {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ()> {
        let path = path.as_ref();
        if !path.is_absolute() {
            return Err(());
        }

        let mut parts = Vec::new();
        let mut path_partial = String::new();
        for el in path.components() {
            path_partial.push_str(&el.as_os_str().to_string_lossy());
            match el {
                std::path::Component::Prefix(_) => todo!(),
                std::path::Component::RootDir => parts.push(("/".to_string(), "/".to_string())),
                std::path::Component::CurDir => todo!(),
                std::path::Component::ParentDir => todo!(),
                std::path::Component::Normal(os_str) => {
                    parts.push((os_str.to_string_lossy().to_string(), path_partial.clone()));
                    path_partial.push('/');
                },
            }
        }

        Ok(Self {
            parts,
            path: path.to_string_lossy().to_string(),
        })
    }

    pub fn get(&self) -> String {
        self.path.clone()
    }
}

impl IntoIterator for CurrentPath {
    type Item = (String, String);

    type IntoIter = IntoIter<(String, String)>;

    fn into_iter(self) -> Self::IntoIter {
        self.parts.into_iter()
    }
}

impl IntoIterator for &CurrentPath {
    type Item = (String, String);

    type IntoIter = IntoIter<(String, String)>;

    fn into_iter(self) -> Self::IntoIter {
        self.parts.clone().into_iter()
    }
}

pub fn btn_centered<'a>(text: impl Into<Cow<'a, str>>, width: impl Into<Length>) -> Button<'a, AppMessage> {
    Button::new(Text::new(text).horizontal_alignment(Horizontal::Center)).padding(4).width(width)
}

// pub fn default_paths<'a>() -> Column<'a, AppMessage> {
//     #[cfg(target_family = "unix")]
//     {
//         Column::new()
//             .push(btn_to_path(std::env::var("HOME").unwrap_or("~/".to_string()), "home"))
//             .push(btn_to_path(if let Ok(s) = std::env::var("HOME") {
//                 format!("{}/.local/share", s)
//             } else {
//                 "~/.local/share".to_string()
//             }, "share"))
//             .padding(4).spacing(4)
//     }
//     #[cfg(target_os = "windows")]
//     {
//         Column::new()
//         .push(btn_to_path("%userprofile%", "home"))
//         .push(btn_to_path("%appdata%", "share"))
//     }
// }

// TODO: more reliable check
pub fn is_mc_save(path: impl AsRef<Path>) -> bool {
    if let Ok(_) = std::fs::read_dir(&path) {
        return false;
    }

    {
        if let Ok(file) = File::open(&path) {
            let mut buffer = [0; 32];
            if let Ok(_) =  GzDecoder::new(&file).read(&mut buffer) {
                return match ParserData::from(buffer.to_vec().as_ref()).parse() {
                    Ok(_) => true,
                    Err(ParseError::EndOfBuffer) => true,
                    Err(_) => false,
                }
            }
        }
    }

    if let Ok(mut file) = File::open(&path) {
        let mut buffer = [0; 32];
        if let Ok(_) = file.read(&mut buffer) {
            return match ParserData::from(buffer.to_vec().as_ref()).parse() {
                Ok(_) => true,
                Err(ParseError::EndOfBuffer) => true,
                Err(_) => false,
            }
        }
    }

    false
}

fn list_dir(path: impl AsRef<Path>, show_hidden: bool) -> Result<Vec<(String, String)>, String> {
    match std::fs::read_dir(path) {
        Ok(dir) => {
            let mut data = Vec::new();
            for i in dir {
                match i {
                    Ok(dir) => {
                        #[cfg(target_os = "windows")]
                        if !show_hidden && dir.metadata().unwrap().file_attributes() & 2 != 0 { continue; }
                        #[cfg(target_family = "unix")]
                        if !show_hidden && dir.file_name().to_string_lossy().starts_with(".") { continue; }
                        let name = dir.file_name();
                        let name = name.to_string_lossy();
                        if dir.file_type().unwrap().is_dir() ||
                            name.to_lowercase().ends_with(".dat") ||
                            name.to_lowercase().ends_with(".nbt") {
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

pub fn dir_buttons<'a>(path: impl Into<Cow<'a, str>> + AsRef<Path> + Clone) -> Column<'a, AppMessage> {
    let mut list = Column::new().padding(4).spacing(4);
    if path.as_ref() != Path::new("/") {
        list = list.push(btn_centered("..", Length::Fill).on_press(AppMessage::TryOpenFile(path.as_ref().parent().unwrap().to_str().unwrap().to_string())));
    }
    match list_dir(path.as_ref(), false) {
        Ok(data) => {
            for entry in data {
                list = list.push(btn_centered(entry.0.clone(), Length::Fill).style(
                    if is_mc_save(&Path::new(&entry.1)) {
                        iced::theme::Button::Positive
                    } else {
                        iced::theme::Button::Primary
                    }
                ).on_press(AppMessage::TryOpenFile(entry.1)));
            }
        },
        Err(e) => {
            list = list.push(Text::new(format!("Cannot open directory:\n{}", e)));
        }
    }

    return list;
}

pub fn labeled_element<'a>(text: impl Into<Cow<'a, str>>, element: Element<'a, AppMessage>) -> Row<'a, AppMessage> {
    Row::new()
        .push(Text::new(text.into()).width(Length::Fixed(80.0)))
        .push(element)
        .align_items(iced::Alignment::Center)
}