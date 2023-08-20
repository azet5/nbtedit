use std::path::PathBuf;

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
}

pub struct Tag {
    name: Option<String>,
    tag: TagType,
}

pub struct NbtFile(Tag);