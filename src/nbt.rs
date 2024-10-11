use std::{fmt::{Display, Formatter}, fs::File, io::Read, path::Path, slice::Iter};

use flate2::read::GzDecoder;
use iced::{alignment::Horizontal, widget::{Button, Column, Row, Text}, Length};

use crate::AppMessage;

#[derive(Debug, Clone)]
pub enum TagType {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<Tag>),
    Compound(Vec<Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl TagType {
    pub fn type_name(&self) -> String {
        match self {
            TagType::End => "TAG_End".to_string(),
            TagType::Byte(_) => "TAG_Byte".to_string(),
            TagType::Short(_) => "TAG_Short".to_string(),
            TagType::Int(_) => "TAG_Int".to_string(),
            TagType::Long(_) => "TAG_Long".to_string(),
            TagType::Float(_) => "TAG_Float".to_string(),
            TagType::Double(_) => "TAG_Double".to_string(),
            TagType::ByteArray(_) => "TAG_ByteArray".to_string(),
            TagType::String(_) => "TAG_String".to_string(),
            TagType::List(_) => "TAG_List".to_string(),
            TagType::Compound(_) => "TAG_Compound".to_string(),
            TagType::IntArray(_) => "TAG_IntArray".to_string(),
            TagType::LongArray(_) => "TAG_LongArray".to_string(),
        }
    }

    pub fn is_compound(&self) -> bool {
        match self {
            TagType::ByteArray(_) |
            TagType::List(_) |
            TagType::Compound(_) |
            TagType::IntArray(_) |
            TagType::LongArray(_) => true,
            _ => false,
        }
    }
}

impl Display for TagType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TagType::End => String::new(),
            TagType::Byte(x) => x.to_string(),
            TagType::Short(x) => x.to_string(),
            TagType::Int(x) => x.to_string(),
            TagType::Long(x) => x.to_string(),
            TagType::Float(x) => x.to_string(),
            TagType::Double(x) => x.to_string(),
            TagType::String(x) => x.to_string(),
            TagType::ByteArray(x) => format!("{} item(s)", x.len()),
            TagType::List(x) => format!("{} item(s)", x.len()),
            TagType::Compound(x) => format!("{} item(s)", x.len()),
            TagType::IntArray(x) => format!("{} item(s)", x.len()),
            TagType::LongArray(x) => format!("{} item(s)", x.len()),
        })
    }
}

#[derive(Debug, Clone)]
pub enum TagMessage {
    ExpandTag(bool),
    SelectTag(Option<String>, Tag),
    EditTag {
        name: Option<String>,
        value: Option<TagType>
    },
    CreateTag {
        name: String,
        tag: TagType,
    },
    RemoveTag,
}

#[derive(Debug, Clone)]
pub struct Tag {
    id: usize,
    name: Option<String>,
    tag: TagType,
    expanded: bool,
    hidden: bool,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            id: 0,
            name: None,
            tag: TagType::End,
            expanded: false,
            hidden: false,
        }
    }
}

impl Tag {
    pub fn get(&self) -> &TagType {
        &self.tag
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn remove(&mut self, id: usize) {
        if let TagType::Compound(tags) = &mut self.tag {
            if let Some(i) = tags.iter_mut().map(|x| {
                x.remove(id);
                x
            }).position(|x| x.id == id) {
                tags.remove(i);
                return;
            }
        }
    }

    pub fn find(&self, id: usize) -> Option<&Self> {
        if self.id == id {
            return Some(self);
        }

        match &self.tag {
            TagType::Compound(tags) |
            TagType::List(tags) => {
                for tag in tags {
                    if let Some(s) = tag.find(id) {
                        return Some(s);
                    }
                }
            },
            _ => {},
        }

        None
    }

    pub fn find_mut(&mut self, id: usize) -> Option<&mut Self> {
        if self.id == id {
            return Some(self);
        }

        match &mut self.tag {
            TagType::Compound(tags) |
            TagType::List(tags) => {
                for tag in tags {
                    if let Some(s) = tag.find_mut(id) {
                        return Some(s);
                    }
                }
            },
            _ => {},
        }

        None
    }

    pub fn update(&mut self, message: TagMessage) {
        match message {
            TagMessage::ExpandTag(expanded) => self.expanded = expanded,
            TagMessage::EditTag {
                name,
                value
            } => {
                self.name = name;

                if let Some(value) = value {
                    self.tag = value;
                }
            },
            TagMessage::RemoveTag => self.hidden = true,
            _ => {},
        }
    }

    pub fn view(&self) -> Column<'_, AppMessage> {
        let mut column = Column::new().padding([0, 12]);
        if !self.hidden {
            column = column.push(Row::new()
                .push(Button::new(
                        Text::new(if self.expanded { "-" } else { "+" })
                        .horizontal_alignment(Horizontal::Center))
                    .width(Length::Fixed(25.0))
                    .on_press_maybe(match self.tag {
                        TagType::Compound(_) |
                        TagType::List(_) |
                        TagType::ByteArray(_) |
                        TagType::IntArray(_) |
                        TagType::LongArray(_) => Some(AppMessage::TagEvent(self.id, TagMessage::ExpandTag(!self.expanded))),
                        _ => None,
                    })
                )
                .push(Button::new(if let Some(s) = self.name.as_ref() { s.as_str() } else { "(empty)" } )
                    .on_press(AppMessage::TagEvent(self.id, TagMessage::SelectTag(self.name.clone(), self.clone())))
                )
            );
    
            if let TagType::Compound(tags) = &self.tag {
                if self.expanded {
                    for tag in tags {
                        column = column.push(tag.view());
                    }
                }
            } else if let TagType::List(tags) = &self.tag {
                if self.expanded {
                    for tag in tags {
                        column = column.push(tag.view());
                    }
                }
            }
        }

        column
    }
}

pub struct NbtFile(Tag);

impl NbtFile {
    pub fn get_mut_tag(&mut self) -> &mut Tag {
        &mut self.0
    }

    pub fn get_tag(&self) -> &Tag {
        &self.0
    }
}

#[derive(Debug)]
pub enum ParseError {
    EndOfBuffer,
    InvalidTagType,
    InvalidPayload,
    InvalidRootTag,
    IOError(String),
}

struct ParserData<'a> {
    bytes: Iter<'a, u8>,
    max_id: usize,
}

impl<'a> ParserData<'a> {
    pub fn from(bytes: &'a Vec<u8>) -> Self {
        Self {
            bytes: bytes.iter(),
            max_id: 0,
        }
    }

    pub fn advance(&mut self) -> Result<u8, ParseError> {
        match self.bytes.next() {
            Some(t) => Ok(*t),
            None => Err(ParseError::EndOfBuffer),
        }
    }

    fn get_type(&mut self) -> Result<u8, ParseError> {
        match self.advance()? {
            0x0d.. => Err(ParseError::InvalidTagType),
            t => Ok(t),
        }
    }

    fn read_utf8_string(&mut self) -> Result<String, ParseError> {
        let name = {
            let upper = self.advance()?;
            let lower = self.advance()?;
            let len = u16::from_be_bytes([upper, lower]);
            let mut name = Vec::with_capacity(len.into());
            for _ in 0..len {
                if let Some(c) = self.bytes.next() {
                    name.push(*c);
                } else {
                    return Err(ParseError::EndOfBuffer);
                }
            }
            name
        };
        
        if let Ok(str) = String::from_utf8(name) {
            Ok(str)
        } else {
            Err(ParseError::InvalidPayload)
        }
    }

    fn read_byte(&mut self) -> Result<TagType, ParseError> {
        let byte = self.advance()?;
        Ok(TagType::Byte(byte as i8))
    }

    fn read_short(&mut self) -> Result<TagType, ParseError> {
        let bytes = [self.advance()?, self.advance()?];
        Ok(TagType::Short(i16::from_be_bytes(bytes)))
    }

    fn read_int(&mut self) -> Result<TagType, ParseError> {
        let bytes = [self.advance()?, self.advance()?, self.advance()?, self.advance()?];
        Ok(TagType::Int(i32::from_be_bytes(bytes)))
    }

    fn read_long(&mut self) -> Result<TagType, ParseError> {
        let bytes = [self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?];
        Ok(TagType::Long(i64::from_be_bytes(bytes)))
    }

    fn read_float(&mut self) -> Result<TagType, ParseError> {
        let bytes = [self.advance()?, self.advance()?, self.advance()?, self.advance()?];
        Ok(TagType::Float(f32::from_be_bytes(bytes)))
    }

    fn read_double(&mut self) -> Result<TagType, ParseError> {
        let bytes = [self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?];
        Ok(TagType::Double(f64::from_be_bytes(bytes)))
    }

    fn read_byte_array(&mut self) -> Result<TagType, ParseError> {
        let length = i32::from_be_bytes([self.advance()?, self.advance()?, self.advance()?, self.advance()?]);
        let mut bytes = Vec::with_capacity(length as usize);
        for _ in 0..length {
            bytes.push(self.advance()? as i8);
        }

        Ok(TagType::ByteArray(bytes))
    }

    fn read_string(&mut self) -> Result<TagType, ParseError> {
        Ok(TagType::String(self.read_utf8_string()?))
    }

    fn read_int_array(&mut self) -> Result<TagType, ParseError> {
        let length = i32::from_be_bytes([self.advance()?, self.advance()?, self.advance()?, self.advance()?]);
        let mut array = Vec::with_capacity(length as usize * 4);
        for _ in 0..length {
            array.push(i32::from_be_bytes([self.advance()?, self.advance()?, self.advance()?, self.advance()?]));
        }

        Ok(TagType::IntArray(array))
    }

    fn read_long_array(&mut self) -> Result<TagType, ParseError> {
        let length = i32::from_be_bytes([self.advance()?, self.advance()?, self.advance()?, self.advance()?]);
        let mut array = Vec::with_capacity(length as usize * 8);
        for _ in 0..length {
            array.push(i64::from_be_bytes([self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?, self.advance()?]));
        }

        Ok(TagType::LongArray(array))
    }

    fn read_list(&mut self) -> Result<TagType, ParseError> {
        let tag_type = self.advance()?;
        let length = i32::from_be_bytes([self.advance()?, self.advance()?, self.advance()?, self.advance()?]);
        let mut list = Vec::with_capacity(length as usize);
        
        for _ in 0..length {
            self.max_id += 1;
            let tag = Tag {
                id: self.max_id,
                tag: match tag_type {
                    0x00 => TagType::End,
                    0x01 => self.read_byte()?,
                    0x02 => self.read_short()?,
                    0x03 => self.read_int()?,
                    0x04 => self.read_long()?,
                    0x05 => self.read_float()?,
                    0x06 => self.read_double()?,
                    0x07 => self.read_byte_array()?,
                    0x08 => self.read_string()?,
                    0x09 => self.read_list()?,
                    0x0a => self.read_compound()?,
                    0x0b => self.read_int_array()?,
                    0x0c => self.read_long_array()?,
                    x => unreachable!("this tag type does not exist: {}", x),
                },
                ..Default::default()
            };

            list.push(tag);
        }

        Ok(TagType::List(list))
    }

    fn read_compound(&mut self) -> Result<TagType, ParseError> {
        let mut tags = Vec::new();
        loop {
            self.max_id += 1;
            match self.get_type()? {
                0x00 => {
                    return Ok(TagType::Compound(tags));
                },
                0x01 => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_byte()?,
                    ..Default::default()
                }),
                0x02 => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_short()?,
                    ..Default::default()
                }),
                0x03 => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_int()?,
                    ..Default::default()
                }),
                0x04 => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_long()?,
                    ..Default::default()
                }),
                0x05 => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_float()?,
                    ..Default::default()
                }),
                0x06 => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_double()?,
                    ..Default::default()
                }),
                0x07 => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_byte_array()?,
                    ..Default::default()
                }),
                0x08 => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_string()?,
                    ..Default::default()
                }),
                0x09 => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_list()?,
                    ..Default::default()
                }),
                0x0a => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_compound()?,
                    ..Default::default()
                }),
                0x0b => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_int_array()?,
                    ..Default::default()
                }),
                0x0c => tags.push(Tag {
                    id: self.max_id,
                    name: Some(self.read_utf8_string()?),
                    tag: self.read_long_array()?,
                    ..Default::default()
                }),
                x => unreachable!("this tag type does not exist: {}", x),
            }
        }
    }

    fn parse(&mut self) -> Result<NbtFile, ParseError> {
        if self.get_type()? == 0x0a {
            Ok(NbtFile(Tag {
                name: Some(self.read_utf8_string()?),
                tag: self.read_compound()?,
                ..Default::default()
            }))
        } else {
            Err(ParseError::InvalidRootTag)
        }
    }

}

impl NbtFile {
    pub fn open(path: impl AsRef<Path>) -> Result<NbtFile, ParseError> {
        match File::open(path) {
            Ok(file) => {
                let mut buf = Vec::new();
                match GzDecoder::new(file).read_to_end(&mut buf) {
                    Ok(_) => ParserData::from(&buf).parse(),
                    Err(e) => Err(ParseError::IOError(e.to_string())),
                }
            },
            Err(e) => Err(ParseError::IOError(e.to_string())),
        }
    }
}