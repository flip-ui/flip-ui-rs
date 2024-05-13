#![no_std]
#![no_main]

// export proc macro
pub use flip_ui_macro::flip_ui;

use core::marker::PhantomData;

use flipperzero::dialogs::{
    DialogFileBrowserOptions, DialogMessage, DialogMessageButton, DialogsApp,
};

pub struct InputDialog<'a> {
    phantom: PhantomData<&'a ()>,
}

pub enum View<'a> {
    Message(DialogMessage<'a>),
    Alert(DialogMessage<'a>),
    Browser(DialogFileBrowserOptions<'a>),
    Input(self::InputDialog<'a>),
}

pub enum Event {
    MessageBack,
    MessageLeft,
    MessageRight,
    MessageCenter,
    AlertOk,
    BrowserSelect,
    Input,
}

impl<'a> View<'a> {
    pub fn show(&self, app: &mut DialogsApp) -> Event {
        match self {
            View::Message(dialog) => match app.show_message(dialog) {
                DialogMessageButton::Back => Event::MessageBack,
                DialogMessageButton::Left => Event::MessageLeft,
                DialogMessageButton::Right => Event::MessageRight,
                DialogMessageButton::Center => Event::MessageCenter,
            },
            View::Alert(dialog) => match app.show_message(dialog) {
                DialogMessageButton::Center => Event::AlertOk,
                _ => unreachable!(),
            },
            View::Browser(_) => Event::BrowserSelect,
            View::Input(_) => Event::Input,
        }
    }
}
