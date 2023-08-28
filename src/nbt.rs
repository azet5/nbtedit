use std::{path::Path, slice::Iter, fs::File, io::Read};

use flate2::read::GzDecoder;

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
    List(Vec<TagType>),
    Compound(Vec<Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

pub struct Tag {
    name: Option<String>,
    tag: TagType,
}

impl Tag {
    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    pub fn get_tag(&self) -> &TagType {
        &self.tag
    }
}

pub struct NbtFile(Tag);

impl NbtFile {
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
    level: i8,
}

impl<'a> ParserData<'a> {
    pub fn from(bytes: &'a Vec<u8>) -> Self {
        Self {
            bytes: bytes.iter(),
            level: 0,
        }
    }

    pub fn advance(&mut self) -> Result<u8, ParseError> {
        match self.bytes.next() {
            Some(t) => {
                eprintln!("{}", t);
                Ok(*t)
            },
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
        match tag_type {
            0x00 => list.push(TagType::End),
            0x01 => list.push(read_byte(data)?),
            0x02 => list.push(read_short(data)?),
            0x03 => list.push(read_int(data)?),
            0x04 => list.push(read_long(data)?),
            0x05 => list.push(read_float(data)?),
            0x06 => list.push(read_double(data)?),
            0x07 => list.push(read_byte_array(data)?),
            0x08 => list.push(read_string(data)?),
            0x09 => list.push(read_list(data)?),
            0x0a => list.push(read_compound(data)?),
            0x0b => list.push(read_int_array(data)?),
            0x0c => list.push(read_long_array(data)?),
            x => unreachable!("this tag type does not exist: {}", x),
        }
    }

    Ok(TagType::List(list))
}

fn read_compound(data: &mut ParserData) -> Result<TagType, ParseError> {
    data.level += 1;
    let mut tags = Vec::new();
    loop {
        match get_type(data)? {
            0x00 => {
                data.level -= 1;
                return Ok(TagType::Compound(tags));
            },
            0x01 => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_byte(data)?,
            }),
            0x02 => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_short(data)?,
            }),
            0x03 => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_int(data)?,
            }),
            0x04 => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_long(data)?,
            }),
            0x05 => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_float(data)?,
            }),
            0x06 => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_double(data)?,
            }),
            0x07 => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_byte_array(data)?,
            }),
            0x08 => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_string(data)?,
            }),
            0x09 => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_list(data)?,
            }),
            0x0a => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_compound(data)?,
            }),
            0x0b => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_int_array(data)?,
            }),
            0x0c => tags.push(Tag {
                name: Some(read_utf8_string(data)?),
                tag: read_long_array(data)?,
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
                    Ok(_) => {
                        eprintln!("{:?}", buf);
                        parse(&mut ParserData::from(&buf))
                    },
                    Err(e) => Err(ParseError::IOError(e.to_string())),
                }
            },
            Err(e) => Err(ParseError::IOError(e.to_string())),
        }
    }
}