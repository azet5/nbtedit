use iced::{Length, widget::{Column, PickList, Row, Scrollable, Space, Text, TextInput}};

use crate::{AppMessage, ToggleCreateMode, helpers::{TagChoice, btn_centered, labeled_element}, nbt::{TagMessage, TagType}, screen::Screen};

pub struct EditScreen;

impl Screen for EditScreen {
    fn view<'a>(&'a self, app: &'a crate::NbtEdit) -> iced::Element<'a, crate::AppMessage, iced::Theme> {
        let mut screen = Column::new().width(Length::FillPortion(2)).padding(8).spacing(8);
        if let Some(t) = app.selected_tag.as_ref() {
            screen = screen.push(labeled_element("Tag Type:", Text::new(t.get().type_name()).into()));
    
            if let Some(_) = t.name() {
                screen = screen.push(labeled_element(
                    "Key",
                    TextInput::new("", &app.temp_name)
                        // .on_input(AppMessage::InputName)
                        .into()
                ));
            }
    
            let content = t.get().to_string();
            if t.get().is_compound() {
                screen = screen.push(labeled_element("Content:", Text::new(content).into()));
            } else {
                screen = screen.push(labeled_element(
                    "Value",
                    TextInput::new("123", &app.temp_value)
                        .on_input(AppMessage::InputValue).into()
                ));
            }
    
            screen = screen.push(Space::with_height(20));
            let mut row = Row::new().spacing(4)
                .push(btn_centered("Apply", 100)
                    .on_press_maybe(if t.get().is_compound() {
                            None
                        } else {
                            Some(AppMessage::TagEvent(t.id(), TagMessage::EditTag {
                                name: t.name().cloned(),
                                value: Some(t.get().replace(&app.temp_value)),
                            }))
                        })
                    // .on_press_maybe(None)
                );
            if let TagType::Compound(_) = t.get() {
                row = row.push(btn_centered("Insert Tag", 100).on_press(AppMessage::ToggleCreate(ToggleCreateMode::Tag)));
            } else if let TagType::List(tags) = t.get() {
                row = row.push(btn_centered("Insert Item", 100).on_press(if tags.len() > 0 {
                    AppMessage::ToggleCreate(ToggleCreateMode::ListItem(tags[0].get().clone()))
                } else {
                    AppMessage::ToggleCreate(ToggleCreateMode::Tag)
                }));
            }
            row = row.push(btn_centered("Delete", 100).on_press(AppMessage::TagEvent(t.id(), TagMessage::RemoveTag)));
            screen = screen.push(row);
        }

        match &app.create_screen {
            ToggleCreateMode::None => {},
            ToggleCreateMode::Tag => {
                screen = screen.push("Insert new tag")
                    .push(labeled_element("Name", TextInput::new(
                        "",
                        app.create_data.as_ref().unwrap().name.as_str()
                    ).on_input(|x| AppMessage::InputTagName(x)).into()))
                    .push(labeled_element("Type", PickList::new(
                        &TagChoice::ALL[..],
                        Some(app.create_data.as_ref().unwrap().tag),
                        |x| AppMessage::InputTagValue(x)).into()))
                    .push(Row::new().spacing(4)
                        .push(btn_centered("Done", 70))
                        .push(btn_centered("Cancel", 70).on_press(AppMessage::ToggleCreate(ToggleCreateMode::None)))   
                    );
            },
            ToggleCreateMode::ListItem(_) => {
                screen = screen.push("Insert new item")
                    .push(labeled_element("Value", TextInput::new(
                        "",
                        app.create_data.as_ref().unwrap().name.as_str()
                    ).into()))
                    .push(Row::new().spacing(4)
                        .push(btn_centered("Done", 70))
                        .push(btn_centered("Cancel", 70).on_press(AppMessage::ToggleCreate(ToggleCreateMode::None)))   
                    );
            },
        }

        Row::new()
            .push(Scrollable::new(app.level_dat.as_ref().unwrap().get_tag().view()).width(Length::FillPortion(2)))
            .push(screen)
            .into()
    }
}