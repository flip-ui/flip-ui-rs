#![allow(clippy::large_enum_variant)]

use std::{ffi::CString, fmt, fs};

use proc_macro as pc;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use serde::Deserialize;
use syn::{parse::Parse, spanned::Spanned};

/// Creates the App struct with the running logic
///
/// ## Helper Functions
/// - `next`
/// - `back`
/// - `close`
///
/// ## Example
/// ```rs
/// flip_ui! {
///     App, // struct name
///     "src/main.json", // ui data
///     next => next, // helper/custom functions
///     back => back,
///     close => close,
///     none => none,
/// }
/// ```
#[proc_macro]
pub fn flip_ui(input: pc::TokenStream) -> pc::TokenStream {
    match flip_ui_inner(input.into()) {
        Ok(result) => result.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn flip_ui_inner(input: TokenStream) -> syn::Result<TokenStream> {
    let span = input.span();
    let args = syn::parse2::<Args>(input)?;

    let s = fs::read_to_string(&args.path).map_err(|_| s_err(span, "JSON not found!"))?;
    let data: Data = serde_json::from_str(&s).map_err(|e| s_err(span, e.to_string()))?;
    let views = data.views;

    let mut handlers = TokenStream::new();
    for (ident, path) in args.handler {
        let name = ident.to_string();
        for (i, page) in views.iter().enumerate() {
            match page {
                View::Message(message) => {
                    if let Some(buttons) = &message.buttons {
                        for (e, button) in buttons.iter().enumerate() {
                            if let Some(b) = button {
                                if b.function == name {
                                    let e = event_from_id(e);
                                    handlers.extend(quote!( (#i, #e) => #path(self), ))
                                }
                            }
                        }
                    }
                    if message.back_function == name {
                        let e = quote!(flip_ui::Event::Back);
                        handlers.extend(quote!( (#i, #e) => #path(self), ))
                    }
                }
                View::Alert(e) => {
                    if e.function == name {
                        let e = quote!(flip_ui::Event::AlertOk);
                        handlers.extend(quote!( (#i, #e) => #path(self), ))
                    } else if e.back_function == name {
                        let e = quote!(flip_ui::Event::Back);
                        handlers.extend(quote!( (#i, #e) => #path(self), ))
                    }
                }
            }
        }
    }

    let page_len = views.len();
    let app_ident = args.ty;

    Ok(quote! {
        /// The app struct inheriting all state
        struct #app_ident<'a> {
            /// Views indexed as an array
            pub views: [flip_ui::View<'a>; #page_len],
            /// The current view index
            pub current: Option<usize>,
            /// Inner Data
            pub inner: flipperzero::dialogs::DialogsApp,
        }

        impl<'a> #app_ident<'a> {
            /// Creating the app inheriting all needed data from the JSON
            fn create() -> Self {
                Self {
                    views: [#(#views),*],
                    current: Some(0),
                    inner: flipperzero::dialogs::DialogsApp::open(),
                }
            }
            /// Run the App
            fn show(&mut self) {
                while let Some(current) = self.current {
                    let event = self.views[current].show(&mut self.inner);
                    match (current, event) {
                        #handlers
                        _ => panic!("Unexpected event"),
                    }
                }
            }
        }

        /// Close the app
        fn close(app: &mut #app_ident) {
            app.current = None;
        }

        /// Advance to the view before
        fn back(app: &mut #app_ident) {
            app.current = Some(app.current.unwrap_or(0) - 1)
        }

        /// Advance to the next view
        fn next(app: &mut #app_ident) {
            app.current = Some(app.current.unwrap_or(0) + 1)
        }

        /// Placeholder event for doing nothing
        fn none(app: &mut #app_ident) {}
    })
}

struct Args {
    ty: syn::Ident,
    path: String,
    handler: Vec<(syn::Ident, syn::Path)>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ty = syn::Ident::parse(input)?;
        <syn::Token![,]>::parse(input)?;
        let path = input.parse::<syn::LitStr>()?.value();
        let mut handler = Vec::new();

        while !input.is_empty() {
            <syn::Token![,]>::parse(input)?;
            if input.is_empty() {
                break;
            }

            let ident = syn::Ident::parse(input)?;
            <syn::Token![=>]>::parse(input)?;
            let path = syn::Path::parse(input)?;

            handler.push((ident, path));
        }

        Ok(Self { ty, path, handler })
    }
}

fn s_err(span: proc_macro2::Span, msg: impl fmt::Display) -> syn::Error {
    syn::Error::new(span, msg)
}

#[derive(Deserialize)]
struct Data {
    views: Vec<View>,
}

#[derive(Deserialize)]
enum View {
    #[serde(rename = "message")]
    Message(MessageData),
    #[serde(rename = "alert")]
    Alert(AlertData),
}

impl ToTokens for View {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            View::Message(MessageData {
                header,
                text,
                buttons,
                ..
            }) => {
                let mut expand = quote! {};
                if let Some(Label {
                    text,
                    x,
                    y,
                    horizontal,
                    vertical,
                }) = header
                {
                    let text = c_str(text);
                    expand
                        .extend(quote! {dialog.set_header(#text, #x, #y, #horizontal, #vertical);})
                }
                if let Some(Label {
                    text,
                    x,
                    y,
                    horizontal,
                    vertical,
                }) = text
                {
                    let text = c_str(text);
                    expand.extend(quote! {dialog.set_text(#text, #x, #y, #horizontal, #vertical);})
                }
                if let Some(buttons) = buttons {
                    let bl = buttons[0].as_ref().map_or(quote! {None}, |b| {
                        let s = c_str(&b.text);
                        quote! { Some(#s) }
                    });
                    let bc = buttons[1].as_ref().map_or(quote! {None}, |b| {
                        let s = c_str(&b.text);
                        quote! { Some(#s) }
                    });
                    let br = buttons[2].as_ref().map_or(quote! {None}, |b| {
                        let s = c_str(&b.text);
                        quote! { Some(#s) }
                    });
                    expand.extend(quote! {dialog.set_buttons(#bl, #bc, #br);})
                }
                tokens.extend(quote! {
                    flip_ui::View::Message({
                        let mut dialog = flipperzero::dialogs::DialogMessage::new();
                        #expand
                        dialog
                    })
                })
            }
            View::Alert(AlertData { text, .. }) => {
                let text = c_str(&text);
                tokens.extend(quote! {
                    flip_ui::View::Alert({
                        let mut dialog = flipperzero::dialogs::DialogMessage::new();

                        dialog.set_text(#text, 0, 0, flipperzero::gui::canvas::Align::Left, flipperzero::gui::canvas::Align::Top);
                        dialog.set_buttons(None, Some(c"OK"), None);

                        dialog
                    })
                })
            }
        }
    }
}

fn c_str(s: &str) -> syn::LitCStr {
    syn::LitCStr::new(&CString::new(s).unwrap(), Span::call_site())
}

#[derive(Deserialize)]
struct MessageData {
    header: Option<Label>,
    text: Option<Label>,
    buttons: Option<[Option<Event>; 3]>,
    back_function: String,
}

#[derive(Deserialize)]
struct AlertData {
    text: String,
    function: String,
    back_function: String,
}

#[derive(Deserialize)]
struct Label {
    text: String,
    x: u8,
    y: u8,
    horizontal: Align,
    vertical: Align,
}

#[derive(Deserialize)]
struct Event {
    text: String,
    function: String,
}

#[derive(Deserialize)]
enum Align {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

impl ToTokens for Align {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Align::Left => tokens.extend(quote!(flipperzero::gui::canvas::Align::Left)),
            Align::Right => tokens.extend(quote!(flipperzero::gui::canvas::Align::Right)),
            Align::Top => tokens.extend(quote!(flipperzero::gui::canvas::Align::Top)),
            Align::Bottom => tokens.extend(quote!(flipperzero::gui::canvas::Align::Bottom)),
            Align::Center => tokens.extend(quote!(flipperzero::gui::canvas::Align::Center)),
        }
    }
}

fn event_from_id(id: usize) -> TokenStream {
    match id {
        0 => quote!(flip_ui::Event::MessageLeft),
        1 => quote!(flip_ui::Event::MessageRight),
        2 => quote!(flip_ui::Event::MessageCenter),
        _ => todo!(),
    }
}
