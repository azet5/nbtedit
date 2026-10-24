use std::{fs::{self, DirEntry}, io, path::{Path, PathBuf}};

use iced::{Element, Length, widget::{Button, Column, Row, Scrollable, Text, button, rule}};

use crate::{AppMessage, helpers::is_mc_save};

#[derive(Debug, Clone)]
pub enum DirMessage {
    ChangePath(PathBuf),
}

#[derive(Debug)]
pub struct DirViewer {
    breadcrumb: DirBreadcrumb,
    listing: DirListing,
}

impl DirViewer {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            breadcrumb: DirBreadcrumb::new(&path),
            listing: DirListing::new(&path),
        }
    }

    pub fn update(&mut self, message: DirMessage) {
        match message {
            DirMessage::ChangePath(path_buf) => {
                self.breadcrumb = DirBreadcrumb::new(&path_buf);
                self.listing = DirListing::new(path_buf);
            },
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        Column::new()
            .push(self.breadcrumb.view())
            .push(rule::horizontal(1))
            .push(self.listing.view())
            .spacing(4)
            .padding(4)
            .into()
    }
}

#[derive(Debug, Clone)]
pub struct PathComponent {
    pub(self) caption: String,
    pub(self) path: PathBuf,
    pub(self) is_nbt: bool,
    pub(self) breadcrumb: bool,
}

impl PathComponent {
    fn view(&self) -> Element<'_, AppMessage> {
        Button::new(self.caption.as_str())
            .on_press(if self.is_nbt {
                AppMessage::TryOpenFile(self.path.clone().into_os_string().into_string().unwrap())
            } else {
                AppMessage::PassToBreadcrumb(DirMessage::ChangePath(self.path.clone()))
            })
            .width(if self.breadcrumb { Length::Shrink } else { 250.into() })
            .style(if self.is_nbt { button::success } else { button::subtle })
            .into()
    }
}

#[derive(Debug)]
pub struct DirBreadcrumb {
    parts: Vec<PathComponent>,
}

impl DirBreadcrumb {
    fn split_path_into_parts(path: impl AsRef<Path>) -> Vec<PathComponent> {
        let mut absolute = PathBuf::new();
        path.as_ref().iter().map(|part| {
            absolute.push(part);
            PathComponent {
                caption: part.to_str().unwrap().to_string(),
                path: absolute.clone(),
                is_nbt: false,
                breadcrumb: true,
            }
        }).collect()
    }

    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            parts: Self::split_path_into_parts(&path),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        Scrollable::new(
            Row::from_vec(self.parts.iter().map(|p| p.view()).collect())
        ).horizontal().spacing(1).into()
    }
}

#[derive(Debug)]
pub struct DirListing {
    listing: Result<Vec<PathComponent>, io::Error>,
}

impl DirListing {
    fn filter_item(item: Result<DirEntry, io::Error>) -> Option<PathComponent> {
        if let Ok(item) = item {
            item.file_type().map_or(None, |file| {
                let is_nbt = is_mc_save(&item.path());
                if file.is_dir() || is_nbt {
                    Some(PathComponent {
                        caption: item.file_name().into_string().unwrap(),
                        path: item.path(),
                        is_nbt,
                        breadcrumb: false,
                    })
                } else { None }
            })
        } else { None }
    }

    fn list_path(path: impl AsRef<Path>) -> Result<Vec<PathComponent>, io::Error> {
        fs::read_dir(path.as_ref()).map(|dir| {
            let mut vec: Vec<PathComponent> = dir.filter_map(|item| {
                Self::filter_item(item)
            }).collect();
            vec.sort_by(|a, b| {
                a.caption.to_lowercase().cmp(&b.caption.to_lowercase())
            });
            vec
        })
    }

    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            listing: Self::list_path(&path),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        Scrollable::new(
            match &self.listing {
                Ok(listing) => Column::from_vec(listing.iter().map(|el| {
                    el.view()
                }).collect()).into(),
                Err(e) => Into::<Element<'_, AppMessage>>::into(Text::new(e.to_string())),
            }
        ).spacing(1).into()
    }
}