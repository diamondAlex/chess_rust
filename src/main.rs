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

#[derive(Debug, Copy, Clone)]
struct ChessSquare{
    piece : ChessPiece,
    color : BColors,
    rec : Rect
}

impl ChessSquare{
    fn new(piece: ChessPiece, color: BColors, rec: Rect) -> Self {
        Self{ piece, color, rec}
    }
    
    fn get_text<'a>(self) -> Vec<Line<'a>>{
        return self.piece.piece.get_text()
    }
}

impl Default for ChessSquare{
    fn default() -> Self{
        ChessSquare{ 
            piece: ChessPiece::default(), 
            color: BColors::BLACK,
            rec : Rect { x:0,y:0,width:0,height:0}
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ChessPiece{
    piece : Piece,
    color : PColors,
}

impl ChessPiece{
    fn new(piece: Piece, color: PColors) -> Self {
        Self{ piece, color}
    }
    
}
impl Default for ChessPiece{
    fn default() -> Self{
        ChessPiece{ 
            piece: Piece::EMPTY, 
            color: PColors::BLACK
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Piece{
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
    EMPTY
}

impl Piece{
    fn get_text_str(self) -> String{
        let lines = self.get_text();
        let mut str : String = String::new();
        for line in lines{
            str.push_str(String::from(line).as_str());
        }
        return str
    }
    fn get_text<'a>(self) -> Vec<Line<'a>>{
        match self{
            Piece::KNIGHT =>{ 
                vec![
                    Line::from("__^"),
                    Line::from("L_*|>"),
                    Line::from("..|_|>"),
                ]
            },
            Piece::ROOK =>{ 
                vec![
                    Line::from("|^^^|"),
                    Line::from(" | |"),
                    Line::from(" |___|"),
                ]
            },
            Piece::QUEEN =>{ 
                vec![
                    Line::from("|\\|/|"),
                    Line::from("\\ /"),
                    Line::from(" |_|"),
                ]
            },
            Piece::BISHOP =>{ 
                vec![
                    Line::from(" .^."),
                    Line::from(" \\ /"),
                    Line::from(" |.|"),
                ]
            },
            Piece::KING =>{ 
                vec![
                    Line::from("|VVV|"),
                    Line::from("\\ /"),
                    Line::from(" |_|"),
                ]
            },
            Piece::PAWN =>{ 
                vec![
                    Line::from(".-."),
                    Line::from("\\ /"),
                    Line::from(" l_l"),
                ]
            },
            Piece::EMPTY =>{
                vec![
                    Line::from(""),
                ]
            }
        }

    }
}

#[derive(Debug, Copy, Clone)]
enum BColors{
    BLACK,
    WHITE
}

impl BColors{
    fn get_color(self) -> Color{
        match self{
            BColors::BLACK => Color::Black,
            BColors::WHITE => Color::White
        }
    }
}
#[derive(Debug, Copy, Clone)]
enum PColors{
    BLACK,
    WHITE
}

impl PColors{
    fn get_color(self) -> Color{
        match self{
            PColors::BLACK => Color::Blue, 
            PColors::WHITE => Color::Red
        }
    }
}

fn print_e_type(input: Piece) -> io::Result<String>{
    match input{
        Piece::PAWN => Ok("it's a pawn".to_string()),
        Piece::KING => Ok("it's a king".to_string()),
        Piece::ROOK => Ok("it's a rook".to_string()),
        Piece::BISHOP => Ok("it's a bishop".to_string()),
        _ => Ok("kindly fock uff".to_string())
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut board : [[ChessSquare;8];8] = [[ChessSquare::default();8];8];
    init_board(&mut board);
    let mut should_quit = false;
    let mut message = String::new();
    let mut from = -1;

    while !should_quit {
        terminal.draw(|f| {
            ui(f,board,&message);
        })?;
        should_quit = handle_events(&mut board,&mut message,&mut from)?;
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;
    Ok(())
}

fn in_square(rect : Rect, x : u16, y: u16) -> bool{
    x >= rect.x
        && x < rect.x + rect.width
        && y >= rect.y
        && y < rect.y + rect.height
}

fn check_square_click(board: &[[ChessSquare;8];8], col: u16, row:u16) -> io::Result<i32>{
    for i in 0..board.len(){
        for j in 0..board[i].len(){
            if in_square(board[i][j].rec, col, row){
                //beautiful
                let x : i32 = i.try_into().unwrap(); 
                let y : i32 = j.try_into().unwrap();
                return Ok((x * 8) + y)
            }
        }
    }
    Ok(-1)
}

fn handle_events( board: &mut[[ChessSquare;8];8], message :&mut String, from: &mut i32) -> io::Result<bool> {
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
                    message.clear();
                    let to = check_square_click(board, mouse.column, mouse.row).unwrap();
                    //*message = format!("{} {}", *from, to);
                    if *from == -1{
                        *from = to;
                        let fi = (*from/8 as i32) as usize;
                        let fj = (*from%8) as usize;
                        *message = print_e_type(board[fi][fj].piece.piece).unwrap();
                    }else{
                        //what the sigma??
                        let ti = (to/8 as i32) as usize;
                        let tj = (to%8) as usize;
                        let fi = (*from/8 as i32) as usize;
                        let fj = (*from%8) as usize;
                        println!("{}.{}.{}.{}",fi,fj,ti,tj);
                        //print_e_type(board[fi][fj].piece.piece);
                        board[ti][tj].piece = board[fi][fj].piece.clone();
                        board[fi][fj].piece = ChessPiece::default();
                        //*message = print_e_type(board[ti][tj].piece.piece).unwrap();
                        *from = -1;
                    }
                }
            },
            _ => (),
        }
    }
    Ok(false)
}

fn get_piece(x : u16, y : u16) -> Piece{
    if x == 1 || x == 6{
        Piece::PAWN
    }else if x == 0 || x == 7{
        if y == 0 || y == 7{
            Piece::ROOK
        }else if y == 1 || y == 6{
            Piece::KNIGHT
        }else if y == 2 || y == 5{
            Piece::BISHOP
        }else if y == 3{
            Piece::QUEEN
        }else if y == 4{
            Piece::KING
        }else{
            Piece::EMPTY
        }
    }else{
        Piece::EMPTY
    }
}
fn get_color(x : u16, y : u16) -> PColors {
    if x < 2{
        PColors::BLACK
    }else{
        PColors::WHITE
    }
}

fn create_new_piece(i : u16, j : u16) -> ChessSquare{
    let x : u16 = 5+(7 * j) as u16;
    let y : u16 = 5+(i * 4) as u16;
    let width : u16 = 7;
    let height : u16 = 4;

    let sp = ChessPiece{
        piece: Piece::PAWN,
        color: PColors::WHITE
    };

    let cs = ChessSquare {
        piece: sp,
        color: {
            if (i*8+(j+i%2))%2 == 1{
                BColors::WHITE
            }else{
                BColors::BLACK
            }
        },
        rec: Rect{x:x,y:y,width:width,height:height}
    };
    return cs
}
fn init_board(board: &mut [[ChessSquare;8];8]){
    for i in 0..8 {
        for j in 0..8{
            let x : u16 = 5+(7*j) as u16;
            let y : u16 = 5+(i*4) as u16;
            let width : u16 = 7;
            let height : u16 = 4;
            
            let sp = ChessPiece{
                piece: get_piece(i,j),
                color: get_color(i,j) 
            };

            board[i as usize][j as usize] = ChessSquare {
                piece: sp,
                color: {
                        if (i*8+(j+i%2))%2 == 1{
                            BColors::WHITE
                        }else{
                            BColors::BLACK
                        }
                },
                rec: Rect{x:x,y:y,width:width,height:height}
            };
        }
    }
}

fn ui(frame: &mut Frame, mut board: [[ChessSquare;8];8], message: &String) {
    for i in 0..8 {
        for j in 0..8{
            let curr_square = board[i][j];
            let block;
            if i==0 {
                block = Block::new()
                    .borders(Borders::BOTTOM | Borders::TOP | Borders::LEFT | Borders::RIGHT)
            }else if i == 7{
                block = Block::new()
                    .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
            }
            else{
                block = Block::new()
                    .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
            }

            let x = curr_square.rec.x;
            let y = curr_square.rec.y;
            let w = curr_square.rec.width;
            let h = curr_square.rec.height;

            let area = Rect::new(x,y,w,h);
            let style = Style::default()
                .bg(curr_square.color.get_color())
                .fg(curr_square.piece.color.get_color());

            let piece = Paragraph::new(curr_square.get_text())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
                .style(style.bold());

            let areaText = Rect::new(5,1,50,3);
            let text = Paragraph::new(message.as_str())
                .block(Block::bordered().title("Test"));

            frame.render_widget(text, areaText);
            //might only need 1 area, cause all could fit in paragraph
            //frame.render_widget(block, area);
            frame.render_widget(piece, area);
        }
    }
}
