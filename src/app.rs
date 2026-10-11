use std::error::Error;

slint::include_modules!();

pub struct App {
    window: MainWindow,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let window = MainWindow::new()?;
        slint::set_xdg_app_id("PinPoint!")?;

        let app = App { window };

        Ok(app)
    }

    pub fn run(self) -> Result<(), slint::PlatformError> {
        self.window.run()
    }
}
