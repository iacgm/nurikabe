use nurikabe::*;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    ui::menu::UI::entry().run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}
