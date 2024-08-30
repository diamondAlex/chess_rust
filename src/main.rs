use std::io::{self, stdout};
use ratatui::{prelude::*, widgets::*};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode,DisableMouseCapture,EnableMouseCapture},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    widgets::{Borders,Paragraph, Block},
    Frame, Terminal,
};

#[derive(Clone)]
enum Piece{
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
    EMPTY
}

struct Board{
     board : Vec<Vec<Piece>>,
     squares : Vec<Vec<Rect>>
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut board : Board =  Board {
        board: vec![vec![Piece::EMPTY;8];8],
        squares: vec![vec![Rect{x:0,y:0,width:0,height:0};8];8]
    };

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|f| {
            ui(f,&mut board);
        })?;
        should_quit = handle_events(&board)?;
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;
    Ok(())
}

fn in_square(rect : &Rect, x : u16, y: u16) -> bool{
    x >= rect.x
        && x < rect.x + rect.width
        && y >= rect.y
        && y < rect.y + rect.height
}

fn check_square_click(board: &Board, col: u16, row:u16) -> io::Result<String>{
    for i in 0..board.squares.len(){
        for j in 0..board.squares[i].len(){
            if in_square(&board.squares[i][j], col, row){
                return Ok(format!("{}.{}", i, j))
            }
        }
    }
    Ok(String::new())
}

fn handle_events(board: &Board) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) => {
                if key.kind != event::KeyEventKind::Press {
                    return Ok(false);
                }
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
            Event::Mouse(mouse) =>{
                //this doesn't seem right, it seems very rustlike(so stupid basically)
                if mouse.kind == event::MouseEventKind::Down(event::MouseButton::Left){
                    println!("{}",check_square_click(&board, mouse.column, mouse.row).unwrap());
                }
            },
            _ => (),
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, board: &mut Board) {
    let b_style = Style::default().fg(Color::Red).bold();
    let w_style = Style::default().fg(Color::White).bold();

    let knight = vec![
        Line::from("__^"),
        Line::from("L_*|>"),
        Line::from("..|_|>"),
    ];
    let rook = vec![
        Line::from("|^^^|"),
        Line::from(" | |"),
        Line::from(" |_|"),
    ];
    let queen = vec![
        Line::from("|\\|/|"),
        Line::from("\\ /"),
        Line::from(" |_|"),
    ];
    let bishop = vec![
        Line::from(" .^."),
        Line::from(" \\ /"),
        Line::from(" |.|"),
    ];
    let king = vec![
        Line::from("|VVV|"),
        Line::from("\\ /"),
        Line::from(" |_|"),
    ];
    let pawn = vec![
        Line::from(".-."),
        Line::from("\\ /"),
        Line::from(" l_l"),
    ];
    for i in 0..8 {
        for j in 0..8{
            let left_block;
            let text; 
            if i == 0 || i == 7 {
                if j == 0 || j ==7 {
                    text = rook.clone();
                }
                else if j == 1 || j ==6{
                    text = knight.clone();
                }
                else if j == 2 || j ==5{
                    text = bishop.clone();
                }
                else if j == 3{
                    text = queen.clone();
                }
                else if j == 4{
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
                    .borders(Borders::BOTTOM | Borders::TOP | Borders::LEFT | Borders::RIGHT)

                }else if i == 7{
                    left_block = Block::new()
                        // don't render the right border because it will be rendered by the right block
                        .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                }
            else{
                left_block = Block::new()
                    .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
                }
            let styled_block;
            let piece;
            if (i*8+(j+i%2))%2 == 1{
                styled_block = left_block.style(Style::new().on_dark_gray());
                piece = Paragraph::new(text)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
            }else{
                styled_block = left_block.style(Style::new().on_light_blue());
                piece = Paragraph::new(text)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
            }
            let new_piece;
            if i <= 1{
                new_piece = piece.style(b_style);
            }else{
                new_piece = piece.style(w_style);
            }
            let x : u16 = 5+(7*j);
    
            let y : u16 = 5+(i*4);
            let width : u16 = 8;
            let height : u16 = 4;
            let area = Rect::new(x,y,width,height);
            board.squares[i as usize][j as usize] = Rect { x:x,y:y,width:width,height:height};

            frame.render_widget(styled_block, area);
            frame.render_widget(new_piece, area);
        }
    }
}
