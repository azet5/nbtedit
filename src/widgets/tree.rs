use iced::{Element, widget::{Button, Column, Component, component, Row}, Renderer};

#[derive(Debug, Clone)]
pub enum TreeMessage {
    NodeExpandChanged(bool),
    NodeSelected,
}

pub struct TreeNode<'a, T> {
    pub children: Vec<TreeNode<'a, T>>,
    pub tag: &'a T,
    pub text: String,
    pub expanded: bool,
}

impl<Message, T> Component<Message, iced::Renderer> for TreeNode<'_, T> {
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
                .push(Button::new("+").on_press_maybe(if self.children.len() > 0 {
                    Some(TreeMessage::NodeExpandChanged(true))
                } else {
                    None
                }))
                .push(Button::new(self.text.as_str())));
            eprintln!("{}", self.expanded);
        } else {
            column = column.push(Row::new()
            .push(Button::new("-").on_press_maybe(if self.children.len() > 0 {
                    Some(TreeMessage::NodeExpandChanged(false))
                } else {
                    None
                }))
                .push(Button::new(self.text.as_str())));
            for child in &self.children {
                column = column.push(Row::new()
                    .push(Button::new("+").on_press_maybe(if child.children.len() > 0 {
                        Some(TreeMessage::NodeExpandChanged(true))
                    } else {
                        None
                    }))
                    .push(Button::new(child.text.as_str())));
            }
            eprintln!("{}", self.expanded);
        }

        column.into()
    }
}

impl<'a, Message: 'a, T> From<TreeNode<'a, T>> for Element<'a, Message, Renderer> {
    fn from(value: TreeNode<'a, T>) -> Self {
        component(value)
    }
}
