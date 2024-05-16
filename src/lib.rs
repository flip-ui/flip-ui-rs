// Generate docs from readme
#![doc = include_str!("../README.md")]
#![no_std]
#![no_main]

// export proc macro
pub use flip_ui_macro::flip_ui;

use core::marker::PhantomData;

use flipperzero::{
    dialogs::{DialogFileBrowserOptions, DialogMessage, DialogMessageButton, DialogsApp},
    furi::string::FuriString,
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
    Back,
    MessageLeft,
    MessageRight,
    MessageCenter,
    AlertOk,
    BrowserSelect(FuriString),
    Input(FuriString),
}

impl<'a> View<'a> {
    pub fn show(&self, app: &mut DialogsApp) -> Event {
        match self {
            View::Message(dialog) => match app.show_message(dialog) {
                DialogMessageButton::Back => Event::Back,
                DialogMessageButton::Left => Event::MessageLeft,
                DialogMessageButton::Right => Event::MessageRight,
                DialogMessageButton::Center => Event::MessageCenter,
            },
            View::Alert(dialog) => match app.show_message(dialog) {
                DialogMessageButton::Center => Event::AlertOk,
                DialogMessageButton::Back => Event::Back,
                _ => unreachable!(),
            },
            // todo: add browser to macro
            View::Browser(dialog) => match app.show_file_browser(None, Some(dialog)) {
                Some(path) => Event::BrowserSelect(path),
                None => Event::Back,
            },
            // todo: add everything
            View::Input(_) => Event::Input(FuriString::new()),
        }
    }
}
