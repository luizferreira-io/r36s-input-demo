use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::joystick::JoystickState;

/// Renders the entire TUI (Text User Interface)
pub fn render_tui(frame: &mut Frame, state: &JoystickState) {
    let size = frame.area();

    // Fill entire screen with blue background
    let background = Block::default()
        .style(Style::default().bg(Color::Blue));
    frame.render_widget(background, size);

    // Create 2-character horizontal margin for bordered block
    let horizontal_margin = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(size);

    // Create 1-line vertical margin for bordered block
    let vertical_margin = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(horizontal_margin[1]);

    let main_area = vertical_margin[1];

    // Main block with blue background and title
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title(" R36S Input Demo (/dev/input/js0) ")
        .style(Style::default().bg(Color::Blue));

    // Main vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),  // Title
            Constraint::Length(3),  // Shoulder buttons
            Constraint::Length(1),  // Blank line
            Constraint::Length(6),  // Pads
            Constraint::Length(1),  // Blank line
            Constraint::Length(5),  // Analog sticks
            Constraint::Min(0),     // Remaining space
        ])
        .split(block.inner(main_area));

    // Render each section
    frame.render_widget(block, main_area);
    render_title(frame, chunks[0]);
    render_shoulder_buttons(frame, chunks[1], state);
    render_pads(frame, chunks[3], state);
    render_analog_sticks(frame, chunks[5], state);
}

/// Renders title with instructions
fn render_title(frame: &mut Frame, area: ratatui::prelude::Rect) {
    let title = Paragraph::new("Press SELECT + START to quit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White).bg(Color::Blue));
    frame.render_widget(title, area);
}

/// Renders L1, L2, R1, R2 buttons
fn render_shoulder_buttons(frame: &mut Frame, area: ratatui::prelude::Rect, state: &JoystickState) {
    let shoulder_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Length(2),
            Constraint::Percentage(50),
        ])
        .split(area);

    // Back Left (L1=button 4, L2=button 6)
    let left_shoulder = create_button_block(
        "Back Left",
        vec![
            (4, "L1"),
            (6, "L2"),
        ],
        state,
        true,
    );

    // Back Right (R2=button 7, R1=button 5)
    let right_shoulder = create_button_block(
        "Back Right",
        vec![
            (7, "R2"),
            (5, "R1"),
        ],
        state,
        true,
    );

    frame.render_widget(left_shoulder, shoulder_chunks[0]);
    frame.render_widget(right_shoulder, shoulder_chunks[2]);
}

/// Renders three pad groups (Left, Control, Right)
fn render_pads(frame: &mut Frame, area: ratatui::prelude::Rect, state: &JoystickState) {
    let pads_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Length(2),
            Constraint::Percentage(34),
            Constraint::Length(2),
            Constraint::Percentage(33),
        ])
        .split(area);

    // Left Pad (D-Pad)
    let left_pad = create_button_block(
        "Left Pad",
        vec![
            (8, "Up   "),
            (9, "Down "),
            (10, "Left "),
            (11, "Right"),
        ],
        state,
        false,
    );

    // Control Pad
    let control_pad = create_button_block(
        "Control Pad",
        vec![
            (16, "FN    "),
            (12, "SELECT"),
            (13, "START "),
        ],
        state,
        false,
    );

    // Right Pad (action buttons)
    let right_pad = create_button_block(
        "Right Pad",
        vec![
            (1, "A"),
            (0, "B"),
            (2, "X"),
            (3, "Y"),
        ],
        state,
        false,
    );

    frame.render_widget(left_pad, pads_chunks[0]);
    frame.render_widget(control_pad, pads_chunks[2]);
    frame.render_widget(right_pad, pads_chunks[4]);
}

/// Renders left and right analog sticks
fn render_analog_sticks(frame: &mut Frame, area: ratatui::prelude::Rect, state: &JoystickState) {
    let analog_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Length(2),
            Constraint::Percentage(50),
        ])
        .split(area);

    // Analog Left (axes 0 and 1, button 14)
    let analog_left = create_analog_block("Analog Left", 14, 0, 1, state);
    
    // Analog Right (axes 2 and 3, button 15)
    let analog_right = create_analog_block("Analog Right", 15, 2, 3, state);

    frame.render_widget(analog_left, analog_chunks[0]);
    frame.render_widget(analog_right, analog_chunks[2]);
}

/// Creates a block with buttons
fn create_button_block<'a>(
    title: &'a str,
    buttons: Vec<(usize, &str)>,
    state: &JoystickState,
    horizontal: bool,
) -> Paragraph<'a> {
    let mut lines = Vec::new();

    if horizontal {
        // Horizontal layout (for shoulder buttons)
        let mut spans = vec![Span::raw(" ")];
        for (i, (btn_id, label)) in buttons.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("  "));
            }
            let icon = create_button_icon(state.buttons[*btn_id]);
            spans.push(icon);
            spans.push(Span::raw(format!(" {}", label)));
        }
        lines.push(Line::from(spans));
    } else {
        // Vertical layout (for pads)
        for (btn_id, label) in buttons.iter() {
            let icon = create_button_icon(state.buttons[*btn_id]);
            lines.push(Line::from(vec![
                Span::raw(" "),
                icon,
                Span::raw(format!(" {}", label)),
            ]));
        }
    }

    Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightCyan))
                .title_style(Style::default().fg(Color::LightCyan))
                .title(title),
        )
        .style(Style::default().fg(Color::White).bg(Color::Blue))
}

/// Creates analog block with axes and press button
fn create_analog_block<'a>(
    title: &'a str,
    press_button: usize,
    x_axis: usize,
    y_axis: usize,
    state: &JoystickState,
) -> Paragraph<'a> {
    let press_icon = create_button_icon(state.buttons[press_button]);

    let lines = vec![
        Line::from(vec![Span::raw(" "), press_icon, Span::raw(" Press")]),
        Line::from(format!(" X: {:6}", state.axes[x_axis])),
        Line::from(format!(" Y: {:6}", state.axes[y_axis])),
    ];

    Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightCyan))
                .title_style(Style::default().fg(Color::LightCyan))
                .title(title),
        )
        .style(Style::default().fg(Color::White).bg(Color::Blue))
}

/// Creates visual icon for button state
fn create_button_icon(pressed: bool) -> Span<'static> {
    if pressed {
        Span::styled("[X]", Style::default().fg(Color::White).bg(Color::Red))
    } else {
        Span::styled("[ ]", Style::default().fg(Color::White).bg(Color::Blue))
    }
}
