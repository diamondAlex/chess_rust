use std::io::{self, stdout};
use ratatui::layout::{Layout, Constraint};
use ratatui::{prelude::*, widgets::*};


use ratatui::{
    backend::CrosstermBackend,
    symbols,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    widgets::{Borders,Paragraph, Block},
    Frame, Terminal,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    let knight = vec![
        Line::from("__^".gray()),
        Line::from("L_*|>".gray()),
        Line::from("..|_|>".gray()),
    ];
    let rook = vec![
        Line::from("|^^^|".gray()),
        Line::from(" | |".gray()),
        Line::from(" |_|".gray()),
    ];
    let queen = vec![
        Line::from("|\\|/|".gray()),
        Line::from("\\ /".gray()),
        Line::from(" |_|".gray()),
    ];
    let bishop = vec![
        Line::from(" .^.".gray()),
        Line::from(" \\ /".gray()),
        Line::from(" |.|".gray()),
    ];
    let king = vec![
        Line::from("|VVV|".gray()),
        Line::from("\\ /".gray()),
        Line::from(" |_|".gray()),
    ];
    let pawn = vec![
        Line::from(".-.".gray()),
        Line::from("\\ /".gray()),
        Line::from(" l_l".gray()),
    ];
    for i in 0..8 {
        for j in 0..8{
            let mut left_block;
            let mut text; 
            if i == 0 || i == 7 {
                if(j == 0 || j ==7){
                    text = rook.clone();
                }
                else if(j == 1 || j ==6){
                    text = knight.clone();
                }
                else if(j == 2 || j ==5){
                    text = bishop.clone();
                }
                else if(j == 3){
                    text = queen.clone();
                }
                else if(j == 4){
                    text = king.clone();
                }else{
                    text = vec![Line::from("")];
                }
            }else if i == 1 || i == 6{
                text = pawn.clone();
            }else{
                text = vec![Line::from("")];
            }
            if i==0 {
                left_block = Block::new()
                    // don't render the right border because it will be rendered by the right block
                    .borders(Borders::BOTTOM | Borders::TOP | Borders::LEFT | Borders::RIGHT);
                }else if i == 7{
                    left_block = Block::new()
                        // don't render the right border because it will be rendered by the right block
                        .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT);
                }
            else{
                left_block = Block::new()
                    // don't render the right border because it will be rendered by the right block
                    .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT);
                }
            let area = Rect::new(5+(7*j), 5+(i*4),8,4);
            let piece = Paragraph::new(text)
                .style(Style::new().white().on_black())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            frame.render_widget(left_block, area);
            frame.render_widget(piece, area);
        }
    }
}
