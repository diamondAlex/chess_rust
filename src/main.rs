//TODO highlight on click piece
//
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
    widgets::{Paragraph, Block},
    Frame, Terminal,
};

#[derive(Debug, Copy, Clone)]
struct ChessPiece{
    piece : Piece,
    color : PColors,
    rec : Rect
}

impl ChessPiece{
    fn new(piece: Piece, color: PColors, rec: Rect) -> Self {
        Self{ piece, color, rec}
    }
    fn check_diagonal_moves(
        &self,
        moves: &mut Vec<String>,
        x: usize,
        y: usize,
        board: &[[ChessPiece; 8]; 8],
    ) {
        // Direction vectors for diagonal movement: (dx, dy)
        let directions: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

        // Iterate through each diagonal direction
        for (dx, dy) in directions.iter() {
            let mut i = 1; // Incremental steps in each direction
            while i < 8 {
                let new_x = x as isize + i * dx;
                let new_y = y as isize + i * dy;

                // Ensure we're within board bounds
                if new_x < 0 || new_x >= 8 || new_y < 0 || new_y >= 8 {
                    break;
                }

                let new_x = new_x as usize;
                let new_y = new_y as usize;

                let target = &board[new_x][new_y];

                if target.piece == Piece::EMPTY {
                    moves.push(format!("{}{}{}{}", x, y, new_x, new_y));
                } else if self.color != target.color {
                    moves.push(format!("{}{}{}{}", x, y, new_x, new_y));
                    break;
                } else {
                    break;
                }

                i += 1;
            }
        }
    }

    fn check_pawn_moves(
        &self,
        moves: &mut Vec<String>,
        x: usize,
        y: usize,
        board: &[[ChessPiece; 8]; 8],
        //have to add en passant
        //en_passant_square: Option<(usize, usize)>,
    ) {
        let direction = if self.color == PColors::WHITE { -1 } else { 1 };
        let start_rank = if self.color == PColors::WHITE { 6 } else { 1 };
        let promotion_rank = if self.color == PColors::WHITE { 0 } else { 7 };

        let new_x = (x as isize + direction) as usize;
        if new_x < 8 && board[new_x][y].piece == Piece::EMPTY {
            if new_x == promotion_rank {
                moves.push(format!("{}{}{}{}Q", x, y, new_x, y)); 
            } else {
                moves.push(format!("{}{}{}{}", x, y, new_x, y));
            }
            if x == start_rank && board[(x as isize + 2 * direction) as usize][y].piece == Piece::EMPTY {
                moves.push(format!("{}{}{}{}", x, y, (x as isize + 2 * direction) as usize, y));
            }
        }

        for &dy in [-1, 1].iter() {
            let new_y = (y as isize + dy) as usize;
            if new_x < 8 && new_y < 8 {
                if board[new_x][new_y].piece != Piece::EMPTY && board[new_x][new_y].color != self.color {
                    if new_x == promotion_rank {
                        moves.push(format!("{}{}{}{}Q", x, y, new_x, new_y)); 
                    } else {
                        moves.push(format!("{}{}{}{}", x, y, new_x, new_y));
                    }
                }

                // En passant capture
                //if let Some((en_passant_x, en_passant_y)) = en_passant_square {
                    //if new_x == en_passant_x && new_y == en_passant_y {
                        //moves.push(format!("{}{}{}{}e", x, y, new_x, new_y)); // 'e' for en passant
                    //}
                //}
            }
        }
    }

    fn check_king_moves(&self, moves: &mut Vec<String>, x: usize, y: usize, board: &[[ChessPiece; 8]; 8]) {
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),  // Diagonal and vertical above
            (0, -1),         (0, 1),     // Horizontal left and right
            (1, -1), (1, 0), (1, 1),     // Diagonal and vertical below
        ];

        for (dx, dy) in directions.iter() {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;

            if new_x < 8 && new_y < 8 {
                let target_piece = board[new_x][new_y];
                if target_piece.piece == Piece::EMPTY || target_piece.color != self.color {
                    moves.push(format!("{}{}{}{}", x, y, new_x, new_y));
                }
            }
        }
    }
    //this is gonna be gross
    fn check_straight_moves(&self, moves: &mut Vec<String>, x: usize, y: usize, board: &[[ChessPiece;8];8]){
        let mut i;
        let mut found = 0;
        if x > 0 {
            i = x -1;
            while i >= 0 {
                if board[i][y].piece == Piece::EMPTY{
                    moves.push(format!("{}{}{}{}",x,y,i,y));
                }else if self.color != board[i][y].color{
                    moves.push(format!("{}{}{}{}",x,y,i,y));
                    break;
                }else{
                    break;
                }
                if i==0{
                    break;
                }
                i-=1;
            }
        }

        if x<7{
            i = x+1;
            while i < 8{
                if board[i][y].piece == Piece::EMPTY{
                    moves.push(format!("{}{}{}{}",x,y,i,y));
                }else if self.color != board[i][y].color{
                    moves.push(format!("{}{}{}{}",x,y,i,y));
                    break;
                }else{
                    break;
                }
                i+=1;
            }
        }

        let mut j;
        if y>0{
            j = y-1 as usize;
            while j >= 0 {
                if board[x][j].piece == Piece::EMPTY{
                    moves.push(format!("{}{}{}{}",x,y,x,j));
                }else if self.color != board[x][j].color{
                    moves.push(format!("{}{}{}{}",x,y,x,j));
                    break;
                }else{
                    break;
                }
                if j==0{
                    break;
                }
                j-=1;
            }
        }

        if y < 7{
            j = y+1;
            while j < 8 {
                if board[x][j].piece == Piece::EMPTY{
                    moves.push(format!("{}{}{}{}",x,y,x,j));
                }else if self.color != board[x][j].color{
                    moves.push(format!("{}{}{}{}",x,y,x,j));
                    break;
                }else{
                    break;
                }
                j+=1;
            }
        }

    }

    //this is all retarded btw, I'm pretty sure x/y are flipped
    fn move_get(&self, x: i32, y: i32, board: &[[ChessPiece;8];8]) -> String {
        let mut moves : Vec<String> = Vec::new();
        match self.piece{
            Piece::KNIGHT =>{
                //beautiful
                if x + 2 < 8 && y + 1 < 8{
                    moves.push(format!("{}{}{}{}",x,y,x+2,y+1));
                }
                if x + 2 < 8 && y - 1 >= 0 {
                    moves.push(format!("{}{}{}{}",x,y,x+2,y-1));
                }
                if x + 1 < 8 && y + 2 < 8{
                    moves.push(format!("{}{}{}{}",x,y,x+1,y+2));
                }
                if x + 1 < 8 && y - 2 >= 0 {
                    moves.push(format!("{}{}{}{}",x,y,x+1,y-2));
                }
                if x - 2 >= 0 && y + 1 < 8{
                    moves.push(format!("{}{}{}{}",x,y,x-2,y+1));
                }
                if x - 2 >= 0 && y - 1 >= 0 {
                    moves.push(format!("{}{}{}{}",x,y,x-2,y-1));
                }
                if x - 1 >= 0 && y + 2 < 8{
                    moves.push(format!("{}{}{}{}",x,y,x-1,y+2));
                }
                if x - 1 >= 0 && y - 2 >= 0{
                    moves.push(format!("{}{}{}{}",x,y,x-1,y-2));
                }
                //land blocked by own piece
            },
            Piece::ROOK =>{
                self.check_straight_moves(&mut moves,x as usize,y as usize,board); 
                //land blocked by own piece
                //path blocked by own piece
            },
            Piece::QUEEN =>{
                self.check_straight_moves(&mut moves,x as usize,y as usize,board); 
                self.check_diagonal_moves(&mut moves,x as usize,y as usize,board); 
            },
            Piece::BISHOP =>{
                self.check_diagonal_moves(&mut moves,x as usize,y as usize,board); 
            },
            Piece::KING =>{
                self.check_king_moves(&mut moves,x as usize,y as usize,board); 
            },
            Piece::PAWN =>{
                self.check_pawn_moves(&mut moves,x as usize,y as usize,board); 
            },
            Piece::EMPTY =>{
            }
        }

        return moves.join(",");
    }
}

//i32 fn add_move(moves: &mut Vec<String>, from: ChessPiece, to: ChessPiece, x : int,y : int, to_x: int, to_y: int){
    //if from.color != to.color{
        //moves.push(format!("{}{}{}{}",x,y,to_x,to_y));
    //}
    //return 
//}

impl Default for ChessPiece{
    fn default() -> Self{
        ChessPiece{ 
            piece: Piece::EMPTY, 
            color: PColors::BLACK,
            rec: Rect{ x:0,y:0,width:0,height:0}
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
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

#[derive(PartialEq, Debug, Copy, Clone)]
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
    let mut board : [[ChessPiece;8];8] = [[ChessPiece::default();8];8];
    init_board(&mut board);
    let mut should_quit = false;
    let mut message = String::new();
    let mut from = -1;
    let mut chess_move = String::new();
    let mut turn : PColors = PColors::WHITE;

    while !should_quit {
        terminal.draw(|f| {
            ui(f,board,&message);
        })?;
        should_quit = handle_events(&mut turn, &mut board,&mut message,&mut from)?;
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

fn check_square_click(board: &[[ChessPiece;8];8], col: u16, row:u16) -> io::Result<i32>{
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

fn handle_events(turn: &mut PColors, board: &mut[[ChessPiece;8];8], message :&mut String, from: &mut i32) -> io::Result<bool> {
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
                if mouse.kind == event::MouseEventKind::Down(event::MouseButton::Left){
                    message.clear();
                    let to = check_square_click(board, mouse.column, mouse.row).unwrap();
                    //*message = format!("{}{}", *from, to);
                    if *from == -1{
                        *from = to;
                        let fi = (*from/8 as i32) as usize;
                        let fj = (*from%8) as usize;
                    }else{
                        //what the sigma??
                        let ti = (to/8 as i32) as usize;
                        let tj = (to%8) as usize;
                        let fi = (*from/8 as i32) as usize;
                        let fj = (*from%8) as usize;

                        *message = board[fi][fj].move_get(fi as i32,fj as i32, &board);
                        
                        if board[fi][fj].color != *turn{
                            *message = String::from("wrong color");
                        }else if !(ti == fi && tj == fj) && message.contains(&format!("{}{}",ti,tj)){
                            *message = if message.contains(&format!("{}{}",ti,tj)) { String::from("true") } else { String::from("false")};
                            let mut piece = board[fi][fj];
                            if piece.piece == Piece::PAWN {
                                if piece.color == PColors::WHITE && ti == 0 {
                                    //promote for white 
                                    board[ti][tj].piece =  Piece::QUEEN;
                                    board[ti][tj].color =  board[fi][fj].color.clone();
                                    board[fi][fj].piece =  Piece::EMPTY;
                                    board[fi][fj].color =  PColors::BLACK;
                                }else if piece.color == PColors::BLACK && ti == 7 {
                                    //promote for black 
                                    board[ti][tj].piece =  Piece::QUEEN;
                                    board[ti][tj].color =  board[fi][fj].color.clone();
                                    board[fi][fj].piece =  Piece::EMPTY;
                                    board[fi][fj].color =  PColors::BLACK;
                                }else{
                                    board[ti][tj].piece =  board[fi][fj].piece.clone();
                                    board[ti][tj].color =  board[fi][fj].color.clone();
                                    board[fi][fj].piece =  Piece::EMPTY;
                                    board[fi][fj].color =  PColors::BLACK;
                                }
                            }else{
                                board[ti][tj].piece =  board[fi][fj].piece.clone();
                                board[ti][tj].color =  board[fi][fj].color.clone();
                                board[fi][fj].piece =  Piece::EMPTY;
                                board[fi][fj].color =  PColors::BLACK;
                            }
                            if *turn == PColors::WHITE{
                                *turn = PColors::BLACK;
                            }else{
                                *turn = PColors::WHITE;
                            }

                        }
                        *from = -1;
                        //*message = if *turn == PColors::WHITE { String::from("white") } else { String::from("black")};
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
fn get_color(x : u16) -> PColors {
    if x < 2{
        PColors::BLACK
    }else{
        PColors::WHITE
    }
}

fn init_board(board: &mut [[ChessPiece;8];8]){
    for i in 0..8 {
        for j in 0..8{
            let x : u16 = 5+(7*j) as u16;
            let y : u16 = 5+(i*4) as u16;
            let width : u16 = 7;
            let height : u16 = 4;
            board[i as usize][j as usize] = ChessPiece {
                piece: get_piece(i,j),
                color: get_color(i),
                rec: Rect{x:x,y:y,width:width,height:height}
            };
        }
    }
}

fn ui(frame: &mut Frame, board: [[ChessPiece;8];8], message: &String) {
    for i in 0..8 {
        for j in 0..8{
            let curr_piece = board[i][j];
            let x = curr_piece.rec.x;
            let y = curr_piece.rec.y;
            let w = curr_piece.rec.width;
            let h = curr_piece.rec.height;

            let area = Rect::new(x,y,w,h);
            let style = Style::default()
                .bg(
                    {
                        if (i*8+(j+i%2))%2 == 1{
                            Color::White
                        }else{
                            Color::Black
                        }
                    }
                )
                .fg(curr_piece.color.get_color());

            let piece = Paragraph::new(curr_piece.piece.get_text())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
                .style(style.bold());

            let area_text = Rect::new(5,1,50,3);
            let text = Paragraph::new(message.as_str())
                .block(Block::bordered().title("Test"));

            frame.render_widget(text, area_text);
            frame.render_widget(piece, area);
        }
    }
}
