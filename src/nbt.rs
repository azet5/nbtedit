use std::{path::Path, slice::Iter, fs::File, io::Read, fmt::{Display, Formatter}};

use flate2::read::GzDecoder;
use iced::widget::{Column, Row, Button};

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

impl Display for TagType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TagType::End => "TAG_End",
            TagType::Byte(_) => "TAG_Byte",
            TagType::Short(_) => "TAG_Short",
            TagType::Int(_) => "TAG_Int",
            TagType::Long(_) => "TAG_Long",
            TagType::Float(_) => "TAG_Float",
            TagType::Double(_) => "TAG_Double",
            TagType::ByteArray(_) => "TAG_ByteArray",
            TagType::String(_) => "TAG_String",
            TagType::List(_) => "TAG_List",
            TagType::Compound(_) => "TAG_Compound",
            TagType::IntArray(_) => "TAG_IntArray",
            TagType::LongArray(_) => "TAG_LongArray",
        })
    }
}

#[derive(Debug, Clone)]
pub enum TagMessage {
    ExpandTag(bool),
    SelectTag(Option<String>, TagType),
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
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            id: 0,
            name: None,
            tag: TagType::End,
            expanded: false,
        }
    }
}

impl Tag {
    pub fn get(&self) -> &TagType {
        &self.tag
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

    pub fn find(&mut self, id: usize) -> Option<&mut Self> {
        if self.id == id {
            return Some(self);
        }

        match &mut self.tag {
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
            _ => {},
        }
    }

    pub fn view(&self) -> Column<'_, AppMessage> {
        let mut column = Column::new().padding([0, 12]);
        column = column.push(Row::new()
            .push(Button::new(if self.expanded { "-" } else { "+" })
                .on_press_maybe(match self.tag {
                    TagType::Compound(_) |
                    TagType::List(_) |
                    TagType::ByteArray(_) |
                    TagType::IntArray(_) |
                    TagType::LongArray(_) => Some(AppMessage::TagEvent(self.id, TagMessage::ExpandTag(!self.expanded))),
                    _ => None,
                })
            )
            .push(Button::new(self.name.as_ref().unwrap().as_str())
                .on_press(AppMessage::TagEvent(self.id, TagMessage::SelectTag(self.name.clone(), self.tag.clone())))
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
                    if let TagType::Compound(tags) = &tag.tag {
                        column = column.push(Row::new()
                            .push(Button::new(if tag.expanded { "-" } else { "+" })
                                .on_press_maybe(if let TagType::Compound(_) = tag.tag {
                                    Some(AppMessage::TagEvent(tag.id, TagMessage::ExpandTag(!tag.expanded)))
                                } else {
                                    None
                                })
                            )
                            .push(Button::new("(empty)")
                                .on_press(AppMessage::TagEvent(tag.id, TagMessage::SelectTag(tag.name.clone(), tag.tag.clone())))
                            ));
                        if tag.expanded {
                            for tag in tags {
                                column = column.push(tag.view());
                            }
                        }
                    } else {
                        column = column.push(Button::new("(empty)")
                            .on_press(AppMessage::TagEvent(tag.id, TagMessage::SelectTag(tag.name.clone(), tag.tag.clone())))
                        );
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
}

fn get_type(data: &mut ParserData) -> Result<u8, ParseError> {
    match data.advance()? {
        0x0d.. => Err(ParseError::InvalidTagType),
        t => Ok(t),
    }
}

fn read_utf8_string(data: &mut ParserData) -> Result<String, ParseError> {
    let name = {
        let upper = data.advance()?;
        let lower = data.advance()?;
        let len = u16::from_be_bytes([upper, lower]);
        let mut name = Vec::with_capacity(len.into());
        for _ in 0..len {
            if let Some(c) = data.bytes.next() {
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

fn read_byte(data: &mut ParserData) -> Result<TagType, ParseError> {
    let byte = data.advance()?;
    Ok(TagType::Byte(byte as i8))
}

fn read_short(data: &mut ParserData) -> Result<TagType, ParseError> {
    let bytes = [data.advance()?, data.advance()?];
    Ok(TagType::Short(i16::from_be_bytes(bytes)))
}

fn read_int(data: &mut ParserData) -> Result<TagType, ParseError> {
    let bytes = [data.advance()?, data.advance()?, data.advance()?, data.advance()?];
    Ok(TagType::Int(i32::from_be_bytes(bytes)))
}

fn read_long(data: &mut ParserData) -> Result<TagType, ParseError> {
    let bytes = [data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?];
    Ok(TagType::Long(i64::from_be_bytes(bytes)))
}

fn read_float(data: &mut ParserData) -> Result<TagType, ParseError> {
    let bytes = [data.advance()?, data.advance()?, data.advance()?, data.advance()?];
    Ok(TagType::Float(f32::from_be_bytes(bytes)))
}

fn read_double(data: &mut ParserData) -> Result<TagType, ParseError> {
    let bytes = [data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?];
    Ok(TagType::Double(f64::from_be_bytes(bytes)))
}

fn read_byte_array(data: &mut ParserData) -> Result<TagType, ParseError> {
    let length = i32::from_be_bytes([data.advance()?, data.advance()?, data.advance()?, data.advance()?]);
    let mut bytes = Vec::with_capacity(length as usize);
    for _ in 0..length {
        bytes.push(data.advance()? as i8);
    }

    Ok(TagType::ByteArray(bytes))
}

fn read_string(data: &mut ParserData) -> Result<TagType, ParseError> {
    Ok(TagType::String(read_utf8_string(data)?))
}

fn read_int_array(data: &mut ParserData) -> Result<TagType, ParseError> {
    let length = i32::from_be_bytes([data.advance()?, data.advance()?, data.advance()?, data.advance()?]);
    let mut array = Vec::with_capacity(length as usize * 4);
    for _ in 0..length {
        array.push(i32::from_be_bytes([data.advance()?, data.advance()?, data.advance()?, data.advance()?]));
    }

    Ok(TagType::IntArray(array))
}

fn read_long_array(data: &mut ParserData) -> Result<TagType, ParseError> {
    let length = i32::from_be_bytes([data.advance()?, data.advance()?, data.advance()?, data.advance()?]);
    let mut array = Vec::with_capacity(length as usize * 8);
    for _ in 0..length {
        array.push(i64::from_be_bytes([data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?, data.advance()?]));
    }

    Ok(TagType::LongArray(array))
}

fn read_list(data: &mut ParserData) -> Result<TagType, ParseError> {
    let tag_type = data.advance()?;
    let length = i32::from_be_bytes([data.advance()?, data.advance()?, data.advance()?, data.advance()?]);
    let mut list = Vec::with_capacity(length as usize);
    
    for _ in 0..length {
        data.max_id += 1;
        let tag = Tag {
            id: data.max_id,
            tag: match tag_type {
                0x00 => TagType::End,
                0x01 => read_byte(data)?,
                0x02 => read_short(data)?,
                0x03 => read_int(data)?,
                0x04 => read_long(data)?,
                0x05 => read_float(data)?,
                0x06 => read_double(data)?,
                0x07 => read_byte_array(data)?,
                0x08 => read_string(data)?,
                0x09 => read_list(data)?,
                0x0a => read_compound(data)?,
                0x0b => read_int_array(data)?,
                0x0c => read_long_array(data)?,
                x => unreachable!("this tag type does not exist: {}", x),
            },
            ..Default::default()
        };

        list.push(tag);
    }

    Ok(TagType::List(list))
}

fn read_compound(data: &mut ParserData) -> Result<TagType, ParseError> {
    let mut tags = Vec::new();
    loop {
        data.max_id += 1;
        match get_type(data)? {
            0x00 => {
                return Ok(TagType::Compound(tags));
            },
            0x01 => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_byte(data)?,
                ..Default::default()
            }),
            0x02 => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_short(data)?,
                ..Default::default()
            }),
            0x03 => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_int(data)?,
                ..Default::default()
            }),
            0x04 => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_long(data)?,
                ..Default::default()
            }),
            0x05 => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_float(data)?,
                ..Default::default()
            }),
            0x06 => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_double(data)?,
                ..Default::default()
            }),
            0x07 => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_byte_array(data)?,
                ..Default::default()
            }),
            0x08 => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_string(data)?,
                ..Default::default()
            }),
            0x09 => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_list(data)?,
                ..Default::default()
            }),
            0x0a => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_compound(data)?,
                ..Default::default()
            }),
            0x0b => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_int_array(data)?,
                ..Default::default()
            }),
            0x0c => tags.push(Tag {
                id: data.max_id,
                name: Some(read_utf8_string(data)?),
                tag: read_long_array(data)?,
                ..Default::default()
            }),
            x => unreachable!("this tag type does not exist: {}", x),
        }
    }
}

fn parse(data: &mut ParserData) -> Result<NbtFile, ParseError> {
    if get_type(data)? == 0x0a {
        Ok(NbtFile(Tag {
            name: Some(read_utf8_string(data)?),
            tag: read_compound(data)?,
            ..Default::default()
        }))
    } else {
        Err(ParseError::InvalidRootTag)
    }
}

impl NbtFile {
    pub fn open(path: impl AsRef<Path>) -> Result<NbtFile, ParseError> {
        match File::open(path) {
            Ok(file) => {
                let mut buf = Vec::new();
                match GzDecoder::new(file).read_to_end(&mut buf) {
                    Ok(_) => parse(&mut ParserData::from(&buf)),
                    Err(e) => Err(ParseError::IOError(e.to_string())),
                }
            },
            Err(e) => Err(ParseError::IOError(e.to_string())),
        }
    }
}