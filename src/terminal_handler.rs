use std::time::{Duration};
use std::io::{self, stdout};
use crossterm::event::KeyEvent;
use crossterm::{
    cursor,
    terminal::{self, ClearType},
    event::{Event, poll, read}
};

pub fn enable_raw_mode() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    Ok(())
}

pub fn disable_raw_mode() -> io::Result<()> {
    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn show_cursor() -> io::Result<()> {
    crossterm::execute!(stdout(), cursor::Show)?;
    Ok(())
}

pub fn reset_cursor_position() -> io::Result<()> {
    crossterm::execute!(stdout(), cursor::MoveTo(0, 0))?;
    Ok(())
}

pub fn prepare_screen() -> io::Result<()> {
    // Hides cursor and clears terminal
    crossterm::execute!(stdout(), cursor::Hide, terminal::Clear(ClearType::All))?;
    Ok(())
}

pub fn clear_screen() -> io::Result<()> {
    crossterm::execute!(stdout(), terminal::Clear(ClearType::All))?;
    Ok(())
}

pub fn get_key_event() -> io::Result<Option<KeyEvent>> {
    if poll(Duration::from_millis(0))? {
        if let Event::Key(key_event) = read()? {
            return Ok(Some(key_event));
        }
    }
    Ok(None)
}
