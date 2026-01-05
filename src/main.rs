mod app;
use std::io;
use app::App;

fn main() -> io::Result<()> {
    let mut terminal: Terminal<CrosstermBackend<>> = ratatui::init();
    
    let mut app: App = App::new();

    let app_result = app.run(&mut terminal);
    
    ratatui.restore();
    app.result
}