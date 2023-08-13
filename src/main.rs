use iced::{Sandbox, Settings, window::{self, PlatformSpecific}};

struct NbtEdit;

impl Sandbox for NbtEdit {
    type Message = ();

    fn new() -> Self {
        NbtEdit
    }

    fn title(&self) -> String {
        format!("nbtedit {}", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, _message: Self::Message) {
        
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        "nbtedit".into()
    }
}

fn main() -> iced::Result {
    NbtEdit::run(Settings {
        window: window::Settings {
            size: (800, 600),
            platform_specific: PlatformSpecific {
                application_id: format!("{}", "CARGO_PKG_NAME")
            },
            ..Default::default()
        },
        ..Default::default()
    })
}
