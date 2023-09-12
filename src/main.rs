#![allow(warnings)]

use std::{
    io::{self, Stdout},
    time::Duration, sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

mod button;
use button::*;

fn main() -> Result<()> {
    let mut terminal = setup_terminal().context("setup failed")?;
    run(&mut terminal).context("app loop failed")?;
    restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen);
    execute!(stdout, EnableMouseCapture);
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    execute!(terminal.backend_mut(), DisableMouseCapture);
    terminal.show_cursor().context("unable to show cursor")
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut input:Event = Event::FocusGained;

    loop {
        if event::poll(Duration::from_millis(100))?{
            input = event::read()?;
        }
        
        match input{
            Event::Key(key) => {
                if key.code == KeyCode::Char('q'){
                    break;
                }
            }
            _ => {}
        }

        terminal.draw(|frame|{
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                ].as_ref())
                .split(frame.size());
            let layout2 = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref())
                .split(layout[1]);

            let para = Paragraph::new("that is a button.\ntry clicking it")
                .alignment(Alignment::Center);
            let para = Arc::new(Mutex::new(para));
            let para_c = Arc::clone(&para);

            let button = Button::default()
                    .text("click me")
                    .left_click(Box::new(move || {
                        *para_c.lock().unwrap() = Paragraph::new("you clicked me")
                            .alignment(Alignment::Center);
                    }));
            frame.render_stateful_widget(button, layout2[1], &mut input);
            frame.render_widget(para.lock().unwrap().clone(), layout2[0]);

        })?;
    }
    Ok(())
}
