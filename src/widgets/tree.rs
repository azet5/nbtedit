use iced::{Element, widget::{Button, Column, Component, component, Row}, Renderer};

use super::WidgetIcon;

#[derive(Debug, Clone)]
pub enum TreeMessage {
    NodeExpandChanged(bool),
    NodeSelected,
}

pub struct TreeNode {
    pub children: Vec<TreeNode>,
    pub icon: WidgetIcon,
    pub text: String,
    pub expanded: bool,
}

impl<Message> Component<Message, iced::Renderer> for TreeNode {
    type Event = TreeMessage;
    type State = ();

    fn update(
            &mut self,
            _state: &mut Self::State,
            event: Self::Event,
        ) -> Option<Message> {
        match event {
            TreeMessage::NodeExpandChanged(value) => {
                self.expanded = value;
                None
            },
            TreeMessage::NodeSelected => None,
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, iced::Renderer> {
        let mut column = Column::new().padding(4).spacing(4);

        if !self.expanded {
            column = column.push(Row::new()
                .push(Button::new("+").on_press(TreeMessage::NodeExpandChanged(true)))
                .push(Button::new(self.text.as_str())));
        } else {
            column = column.push(Row::new()
                .push(Button::new("-").on_press(TreeMessage::NodeExpandChanged(false)))
                .push(Button::new(self.text.as_str())));
            for child in &self.children {
                column = column.push(Row::new()
                    .push(Button::new("+").on_press(TreeMessage::NodeExpandChanged(true)))
                    .push(Button::new(child.text.as_str())));
            }
        }

        column.into()
    }
}

impl<'a, Message: 'a> From<TreeNode> for Element<'a, Message, Renderer> {
    fn from(value: TreeNode) -> Self {
        component(value)
    }
}
