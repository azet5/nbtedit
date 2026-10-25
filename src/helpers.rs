#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::{borrow::Cow, fmt::{Display, Formatter}, fs::File, io::Read, path::Path};

use flate2::read::GzDecoder;
use iced::{Element, Length, Theme, alignment::Horizontal, widget::{Button, Column, Row, Text, button}};

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

pub fn btn_centered<'a>(text: impl Into<Cow<'a, str>> + iced::widget::text::IntoFragment<'a>, width: impl Into<Length>) -> Button<'a, AppMessage> {
    Button::new(Text::new(text).align_x(Horizontal::Center)).padding(4).width(width)
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

pub fn labeled_element<'a>(text: impl Into<Cow<'a, str>>, element: Element<'a, AppMessage>) -> Row<'a, AppMessage> {
    Row::new()
        .push(Text::new(text.into()).width(Length::Fixed(80.0)))
        .push(element)
        .align_y(iced::Alignment::Center)
}

pub fn label_cmp<'a>(text1: impl Into<Cow<'a, str>>, text2: Option<impl Into<Cow<'a, str>>>) -> Column<'a, AppMessage> {
    let text1 = text1.into().to_string();
    Column::new()
        .push(Text::new(text1.clone()))
        .push(if let Some(t) = text2 {
            let text2 = t.into().to_string();
            if text1 != text2.clone() {
                Some(Text::new(format!(" -> {}", text2)))
            } else { None }
        } else { None })
}

pub fn btn_active(theme: &Theme, _status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    button::Style {
        background: Some(iced::Background::Color(palette.background.weak.color)),
        text_color: palette.background.weakest.text,
        ..Default::default()
    }
}