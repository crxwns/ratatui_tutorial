use std::io::Result;

use app::App;

mod app;
mod tui;

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
