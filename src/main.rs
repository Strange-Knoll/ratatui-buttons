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

// boilerplate terminal setup
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen);
    execute!(stdout, EnableMouseCapture);
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}
// boilerplate terminal restore
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    execute!(terminal.backend_mut(), DisableMouseCapture);
    terminal.show_cursor().context("unable to show cursor")
}

//main loop
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {

    let mut input:Option<Event> = None;
    let mut text = "that is a button.\ntry clicking it";
    let text = Arc::new(Mutex::new(text));
    
    loop {
        //poll input events and store them in input
        if event::poll(Duration::from_millis(100))?{
            input = Some(event::read()?);
        }
        
        //read input
        match input{
            //read key events
            Some(Event::Key(key)) => {
                //quit on q
                if key.code == KeyCode::Char('q'){
                    break;
                }
            }
            None => {input = None;}
            _ => {}
        }

        terminal.draw(|frame|{

            //base layout
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                ].as_ref())
                .split(frame.size());
            //layout for button and paragraph
            let layout2 = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref())
                .split(layout[1]);
            //layout for buttons
            let layout3 = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ].as_ref())
                    .split(layout2[1]);


            //clone text to use in btn closure
            let text_c = Arc::clone(&text);
            //button
            let button = Button::default()
                    // set the text of the button
                    .text("default button,\nclick me")
                    // set the alignment of the text
                    .alignment(Alignment::Center)
                    // define left click action with boxed closure
                    .left_click(Box::new(move || {
                        *text_c.lock().unwrap() = "you clicked the button";
                    }));
            //render button which takes the input we stored earlier as its state
            frame.render_stateful_widget(button, layout3[0], &mut input);

            //clone text to use in btn closure
            let text_c2 = Arc::clone(&text);
            //button
            let button2 = Button::default()
                    .text("custom button,\nno, click me")
                    .alignment(Alignment::Center)
                    //here we set the normal, hovered, 
                    //and pressed blocks for this button
                    .normal_block(Block::default()
                        .border_type(BorderType::Rounded)
                        .borders(Borders::ALL))
                    .hovered_block(Block::default()
                        .border_type(BorderType::Thick)
                        .borders(Borders::ALL))
                    .pressed_block(Block::default()
                        .border_type(BorderType::Plain)
                        .borders(Borders::ALL))
                    .left_click(Box::new(move || {
                        *text_c2.lock().unwrap() = "you made the right choice";
                    }));
            frame.render_stateful_widget(button2, layout3[1], &mut input);
            
            
            let para = Paragraph::new(*text.lock().unwrap())
                .alignment(Alignment::Center);
            frame.render_widget(para, layout2[0]);

        })?;
    }
    Ok(())
}
