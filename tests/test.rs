#![no_main]
#![no_std]

use flip_ui::flip_ui;

flip_ui! {
    MyApp,
    "tests/main.json",
    close => close,
    next => next,
    back => back,
    none => none,
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
