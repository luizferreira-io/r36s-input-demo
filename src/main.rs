mod joystick;
mod tui;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, stdout};

use joystick::{JoystickReader, JoystickState};
use tui::render_tui;

const JOYSTICK_DEVICE: &str = "/dev/input/js0";

fn main() -> io::Result<()> {
    // Initialize terminal
    let mut terminal = setup_terminal()?;
    
    // Open joystick device
    let mut joystick = match JoystickReader::open(JOYSTICK_DEVICE) {
        Ok(js) => js,
        Err(e) => {
            cleanup_terminal(&mut terminal)?;
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Failed to open {}: {}", JOYSTICK_DEVICE, e),
            ));
        }
    };

    // Joystick state
    let mut state = JoystickState::new();
    let mut running = true;

    // Main loop
    while running {
        // Process joystick events
        process_joystick_events(&mut joystick, &mut state)?;

        // Render TUI
        terminal.draw(|frame| render_tui(frame, &state))?;

        // Check exit combo (SELECT + START)
        if state.is_exit_combo_pressed() {
            running = false;
        }
    }

    // Clean up terminal
    cleanup_terminal(&mut terminal)?;
    Ok(())
}

/// Sets up terminal for TUI mode
fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

/// Restores terminal to normal state
fn cleanup_terminal(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

/// Processes all pending joystick events
fn process_joystick_events(
    joystick: &mut JoystickReader,
    state: &mut JoystickState,
) -> io::Result<()> {
    while let Some(event) = joystick.read_event()? {
        state.update(&event);
    }

    Ok(())
}

