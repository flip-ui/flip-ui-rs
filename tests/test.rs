#![no_main]
#![no_std]

use flip_ui_macro::flipper_ui;

flipper_ui! {
    MyApp,
    "tests/main.json",
    first => close,
    second => next,
    third => back,
}

#[flipperzero_test::tests]
pub mod tests {
    use crate::MyApp;

    #[test]
    fn test() {
        let mut app = MyApp::create();
        app.show();
    }
}

flipperzero_test::tests_runner!(name = "Some tests", [crate::tests]);
