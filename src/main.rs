use std::{io, usize};
use std::str::FromStr;
use std::io::Write;

#[derive(Debug,Clone)]
enum Piece{
    Empty,
    
    BPawn,
    BBishop,
    BRook,
    BKing,
    BQueen,
    BHorse,
    
    WPawn,
    WBishop,
    WRook,
    WKing,
    WQueen,
    WHorse,

}


fn main() {
    let mut board: Vec<Vec<Piece>>= vec![
        vec![Piece::BRook,Piece::BHorse,Piece::BBishop,Piece::BQueen,Piece::BKing,Piece::BBishop,Piece::BHorse,Piece::BRook],
        vec![Piece::BPawn,Piece::BPawn,Piece::BPawn,Piece::BPawn,Piece::BPawn,Piece::BPawn,Piece::BPawn,Piece::BPawn,],
        vec![Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,],
        vec![Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,],
        vec![Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,],
        vec![Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,Piece::Empty,],
        vec![Piece::WPawn,Piece::WPawn,Piece::WPawn,Piece::WPawn,Piece::WPawn,Piece::WPawn,Piece::WPawn,Piece::WPawn,],
        vec![Piece::WRook,Piece::WHorse,Piece::WBishop,Piece::WQueen,Piece::WKing,Piece::WBishop,Piece::WHorse,Piece::WRook],];


    loop {

        print(&board);
        println!("W H I T E");
        println!("Which piece do you want to move?");
        
        let x = read_letter();
        let y = read_number("Y");
    
        println!("Where do you want to move it?");
        
        let x_m = read_letter();
        let y_m = read_number("Y");
    
        println!("Move from ({}, {}) to ({}, {})", x, y, x_m, y_m);



        move_piece(&mut board, (y-1,x-1), (y_m-1,x_m-1));

        print(&board);
        println!("B L A C K");
        println!("Which piece do you want to move?");
        
        let x = read_letter();
        let y = read_number("Y");
    
        println!("Where do you want to move it?");
        
        let x_m = read_letter();
        let y_m = read_number("Y");
    
        println!("Move from ({}, {}) to ({}, {})", x, y, x_m, y_m);

        move_piece(&mut board, (y-1,x-1), (y_m-1,x_m-1));

    }
    
   }

   fn read_letter() -> usize {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim(); 
    let letter = input.chars().next().unwrap(); 
    let letter_ascii = letter as u8;
    let result = letter_ascii - 96;

    result as usize 
}

fn read_number(coordenada: &str) -> usize {
    loop {
        print!("{}: ", coordenada);
        io::stdout().flush().unwrap(); 
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match usize::from_str(input) {
            Ok(num) => return num,
            Err(_) => println!("Error: invalid input, try again"),
        }
    }
}


fn print(board: &Vec<Vec<Piece>>) {
    // Imprimir la fila superior con las letras de las columnas (a-h)
    print!("     ");
    for letter in b'a'..=b'h' {
        print!("{}   ", letter as char); // Espacio adicional para mejorar la alineación
    }
    println!(""); // Salto de línea

    // Imprimir línea de separación
    println!("   +---+---+---+---+---+---+---+---+");

    // Imprimir las filas numeradas (1-8)
    for i in 0..8 {
        print!(" {} |", i + 1); // Imprimir el número de fila

        for j in 0..8 {
            match board[i][j] {
                Piece::WPawn => print!(" ♟ |"),
                Piece::BPawn => print!(" ♙ |"),
                Piece::WRook => print!(" ♜ |"),
                Piece::BRook => print!(" ♖ |"),
                Piece::WBishop => print!(" ♝ |"),
                Piece::BBishop => print!(" ♗ |"),
                Piece::WHorse => print!(" ♞ |"),
                Piece::BHorse => print!(" ♘ |"),
                Piece::WQueen => print!(" ♛ |"),
                Piece::BQueen => print!(" ♕ |"),
                Piece::WKing => print!(" ♚ |"),
                Piece::BKing => print!(" ♔ |"),
                Piece::Empty => print!("   |"), // Espacio vacío para casillas sin piezas
            }
        }

        println!(); // Salto de línea después de cada fila
        println!("   +---+---+---+---+---+---+---+---+"); // Línea de separación entre filas
    }
}


fn verify(piece: (usize, usize),where_to_move: (usize,usize), board: &Vec<Vec<Piece>>) -> bool {
    let y = piece.0;
    let x = piece.1;
    println!("{x},{y}");
    let piece = board[y][x].clone();
    dbg!(&piece);
    //posible_actions (y,x)
    //board (x,y)
    let mut posible_actions: Vec<(usize,usize)> = vec![];
    
    if let Piece::BPawn = piece {
        if x > 0 && y < 7{
            let variant = format!("{:?}",board[y+1][x-1]);
            if variant.starts_with("W") {
                posible_actions.push((y+1,x-1));
            }
        }
        if x < 7 && y < 7 {
            let variant = format!("{:?}",board[y+1][x+1]);
            if variant.starts_with("W") {
                posible_actions.push((y+1,x+1));
            }
        }
        if y < 7 {
            if let Piece::Empty = board[y+1][x]{
                posible_actions.push((y+1,x));
            }
        }
        if y == 1{
            if let Piece::Empty = board[y+2][x]{
                posible_actions.push((y+2,x));
            }
        }
    }
    
    if let Piece::WPawn = piece {
        if x > 0 && y > 0{
            let variant = format!("{:?}",board[y-1][x-1]);
            if variant.starts_with("B") {
                posible_actions.push((y-1,x-1));
            }
        }
        if x < 7 && y > 0 {
            let variant = format!("{:?}",board[y-1][x+1]);
            if variant.starts_with("B") {
                posible_actions.push((y-1,x+1));
            }
        }

        if y > 0 {
            if let Piece::Empty = board[y-1][x]{
                posible_actions.push((y-1,x));
            }
        }
        if y == 6{
            if let Piece::Empty = board[y-2][x]{
                posible_actions.push((y-2,x));
            }
        }
    }

    if let Piece::BBishop = piece{
        'way: for i in 1..6{
            if x+i < 8 && y+i < 8{
                if let Piece::Empty = board[y+i][x+i]{
                    posible_actions.push((y+i,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x+i]);
                if variant.starts_with("W"){
                    posible_actions.push((y+i,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6{
            if x+i < 8 && y as i32 - i as i32 >= 0{
                if let Piece::Empty = board[y-i][x+i]{
                    posible_actions.push((y-i,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x+i]);
                if variant.starts_with("W"){
                    posible_actions.push((y-i,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32-i as i32 >= 0 && y+i < 8{
                if let Piece::Empty = board[y+i][x-i]{
                    posible_actions.push((y+i,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x-i]);
                if variant.starts_with("W"){
                    posible_actions.push((y+i,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32-i as i32 >= 0 && y as i32-i as i32 >= 0{
                if let Piece::Empty = board[y-i][x-i]{
                    posible_actions.push((y-i,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x-i]);
                if variant.starts_with("W"){
                    posible_actions.push((y-i,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
    }
    if let Piece::WBishop = piece{
        'way: for i in 1..6 {
            if x+i < 8 && y+i < 8{
                if let Piece::Empty = board[y+i][x+i]{
                    posible_actions.push((y+i,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x+i]);
                if variant.starts_with("B"){
                    posible_actions.push((y+i,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x+i < 8 && y as i32 - i as i32 >= 0{
                if let Piece::Empty = board[y-i][x+i]{
                    posible_actions.push((y-i,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x+i]);
                if variant.starts_with("B"){
                    posible_actions.push((y-i,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32 - i as i32 >= 0 && y + i < 8{
                if let Piece::Empty = board[y+i][x-i]{
                    posible_actions.push((y+i,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x-i]);
                if variant.starts_with("B"){
                    posible_actions.push((y+i,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32 - i as i32 >= 0 && y as i32 - i as i32 >=0{
                if let Piece::Empty = board[y-i][x-i]{
                    posible_actions.push((y-i,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x-i]);
                if variant.starts_with("B"){
                    posible_actions.push((y-i,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
    }


    if let Piece::BRook = piece{
        'way: for i in 1..6{
            if x+i < 8{
                if let Piece::Empty = board[y][x+i]{
                    posible_actions.push((y,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y][x+i]);
                if variant.starts_with("W"){
                    posible_actions.push((y,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6{
            if y as i32 - i as i32 >= 0{
                if let Piece::Empty = board[y-i][x]{
                    posible_actions.push((y-i,x));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x]);
                if variant.starts_with("W"){
                    posible_actions.push((y-i,x));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32-i as i32 >= 0{
                if let Piece::Empty = board[y][x-i]{
                    posible_actions.push((y,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y][x-i]);
                if variant.starts_with("W"){
                    posible_actions.push((y,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if y+i < 8{
                if let Piece::Empty = board[y+i][x]{
                    posible_actions.push((y+i,x));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x]);
                if variant.starts_with("W"){
                    posible_actions.push((y+i,x));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
    }

    if let Piece::WRook = piece{
        'way: for i in 1..6{
            if x+i < 8{
                if let Piece::Empty = board[y][x+i]{
                    posible_actions.push((y,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y][x+i]);
                if variant.starts_with("B"){
                    posible_actions.push((y,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6{
            if y as i32 - i as i32 >= 0{
                if let Piece::Empty = board[y-i][x]{
                    posible_actions.push((y-i,x));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x]);
                if variant.starts_with("B"){
                    posible_actions.push((y-i,x));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32-i as i32 >= 0{
                if let Piece::Empty = board[y][x-i]{
                    posible_actions.push((y,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y][x-i]);
                if variant.starts_with("B"){
                    posible_actions.push((y,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if y+i < 8{
                if let Piece::Empty = board[y+i][x]{
                    posible_actions.push((y+i,x));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x]);
                if variant.starts_with("B"){
                    posible_actions.push((y+i,x));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
    }

    if let Piece::BHorse = piece {
        if x+1 < 8 && y + 2 < 8{
            let variant = format!("{:?}",&board[y+2][x+1]);
            if variant.starts_with("W") || variant.starts_with("E"){
                posible_actions.push((y+2,x+1));
            }
        }
        if x as i32 - 1 >= 0 && y +  2 < 8 {
            let variant = format!("{:?}",&board[y+2][x-1]);
            if variant.starts_with("W") || variant.starts_with("E"){
                posible_actions.push((y+2,x-1));
            }
        }
        if x+1 < 8 && y as i32 - 2 >= 0 {if let Piece::WPawn = piece {
        if x > 0 && y > 0{
            let variant = format!("{:?}",board[y-1][x-1]);
            if variant.starts_with("B") {
                posible_actions.push((y-1,x-1));
            }
        }
        if x < 8 && y > 0 {
            let variant = format!("{:?}",board[y-1][x+1]);
            if variant.starts_with("B") {
                posible_actions.push((y-1,x+1));
            }
        }

        if y > 0 {
            if let Piece::Empty = board[y-1][x]{
                posible_actions.push((y-1,x));
            }
        }
        if y == 6{
            if let Piece::Empty = board[y-2][x]{
                posible_actions.push((y-2,x));
            }
        }
    }
            let variant = format!("{:?}",&board[y-2][x+1]);
            if variant.starts_with("W") || variant.starts_with("E"){
                posible_actions.push((y-2,x+1));
            }
        }    
        if x as i32 - 1 >= 0 && y as i32 - 2 >= 0 {
            let variant = format!("{:?}",&board[y-2][x-1]);
            if variant.starts_with("W") || variant.starts_with("E"){
                posible_actions.push((y-2,x-1));
            }
        }
        if y+1 < 8 && x + 2 < 8 {
            let variant = format!("{:?}",&board[y+1][x+2]);
            if variant.starts_with("W") || variant.starts_with("E") {
                posible_actions.push((y+1,x+2));
            }
        }
        if y as i32 - 1 >= 0 && x + 2 < 8 {
            let variant = format!("{:?}",&board[y-1][x+2]);
            if variant.starts_with("W") || variant.starts_with("E") {
                posible_actions.push((y-1,x+2));
            }
        }
        if y+1 < 8 && x as i32 - 2 >= 0 {
            let variant = format!("{:?}",&board[y+1][x-2]);
            if variant.starts_with("W") || variant.starts_with("E") {
                posible_actions.push((y+1,x-2));
            }
        }
        if y as i32 - 1 >= 0 && x as i32 - 2 >= 0 {
            let variant = format!("{:?}",&board[y-1][x-2]);
            if variant.starts_with("W") || variant.starts_with("E") {
                posible_actions.push((y-1,x-2));
            }
        }
    }

    if let Piece::WHorse = piece {
        if x+1 < 8 && y + 2 < 8{
            let variant = format!("{:?}",&board[y+2][x+1]);
            if variant.starts_with("B") || variant.starts_with("E"){
                posible_actions.push((y+2,x+1));
            }
        }
        if x as i32 - 1 >= 0 && y +  2 < 8 {
            let variant = format!("{:?}",&board[y+2][x-1]);
            if variant.starts_with("B") || variant.starts_with("E"){
                posible_actions.push((y+2,x-1));
            }
        }
        if x+1 < 8 && y as i32 - 2 >= 0 {
            let variant = format!("{:?}",&board[y-2][x+1]);
            if variant.starts_with("B") || variant.starts_with("E"){
                posible_actions.push((y-2,x+1));
            }
        }    
        if x as i32 - 1 >= 0 && y as i32 - 2 >= 0 {
            let variant = format!("{:?}",&board[y-2][x-1]);
            if variant.starts_with("B") || variant.starts_with("E"){
                posible_actions.push((y-2,x-1));
            }
        }
        if y+1 < 8 && x + 2 < 8 {
            let variant = format!("{:?}",&board[y+1][x+2]);
            if variant.starts_with("B") || variant.starts_with("E") {
                posible_actions.push((y+1,x+2));
            }
        }
        if y as i32 - 1 >= 0 && x + 2 < 8 {
            let variant = format!("{:?}",&board[y-1][x+2]);
            if variant.starts_with("B") || variant.starts_with("E") {
                posible_actions.push((y-1,x+2));
            }
        }
        if y+1 < 8 && x as i32 - 2 >= 0 {
            let variant = format!("{:?}",&board[y+1][x-2]);
            if variant.starts_with("B") || variant.starts_with("E") {
                posible_actions.push((y+1,x-2));
            }
        }
        if y as i32 - 1 >= 0 && x as i32 - 2 >= 0 {
            let variant = format!("{:?}",&board[y-1][x-2]);
            if variant.starts_with("B") || variant.starts_with("E") {
                posible_actions.push((y-1,x-2));
            }
        }
    }

    if let Piece::BQueen = piece {
        'way: for i in 1..6{
            if x+i < 8{
                if let Piece::Empty = board[y][x+i]{
                    posible_actions.push((y,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y][x+i]);
                if variant.starts_with("W"){
                    posible_actions.push((y,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6{
            if y as i32 - i as i32 >= 0{
                if let Piece::Empty = board[y-i][x]{
                    posible_actions.push((y-i,x));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x]);
                if variant.starts_with("W"){
                    posible_actions.push((y-i,x));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32-i as i32 >= 0{
                if let Piece::Empty = board[y][x-i]{
                    posible_actions.push((y,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y][x-i]);
                if variant.starts_with("W"){
                    posible_actions.push((y,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if y+i < 8{
                if let Piece::Empty = board[y+i][x]{
                    posible_actions.push((y+i,x));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x]);
                if variant.starts_with("W"){
                    posible_actions.push((y+i,x));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }

    'way: for i in 1..6{
            if x+i < 8 && y+i < 8{
                if let Piece::Empty = board[y+i][x+i]{
                    posible_actions.push((y+i,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x+i]);
                if variant.starts_with("W"){
                    posible_actions.push((y+i,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6{
            if x+i < 8 && y as i32 - i as i32 >= 0{
                if let Piece::Empty = board[y-i][x+i]{
                    posible_actions.push((y-i,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x+i]);
                if variant.starts_with("W"){
                    posible_actions.push((y-i,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32-i as i32 >= 0 && y+i < 8{
                if let Piece::Empty = board[y+i][x-i]{
                    posible_actions.push((y+i,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x-i]);
                if variant.starts_with("W"){
                    posible_actions.push((y+i,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32-i as i32 >= 0 && y as i32-i as i32 >= 0{
                if let Piece::Empty = board[y-i][x-i]{
                    posible_actions.push((y-i,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x-i]);
                if variant.starts_with("W"){
                    posible_actions.push((y-i,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
    }

    if let Piece::WQueen = piece {
        'way: for i in 1..6{
            if x+i < 8{
                if let Piece::Empty = board[y][x+i]{
                    posible_actions.push((y,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y][x+i]);
                if variant.starts_with("B"){
                    posible_actions.push((y,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6{
            if y as i32 - i as i32 >= 0{
                if let Piece::Empty = board[y-i][x]{
                    posible_actions.push((y-i,x));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x]);
                if variant.starts_with("B"){
                    posible_actions.push((y-i,x));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
            
        }
        'way: for i in 1..6 {
            if x as i32-i as i32 >= 0{
                if let Piece::Empty = board[y][x-i]{
                    posible_actions.push((y,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y][x-i]);
                if variant.starts_with("B"){
                    posible_actions.push((y,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if y+i < 8{
                if let Piece::Empty = board[y+i][x]{
                    posible_actions.push((y+i,x));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x]);
                if variant.starts_with("B"){
                    posible_actions.push((y+i,x));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x+i < 8 && y+i < 8{
                if let Piece::Empty = board[y+i][x+i]{
                    posible_actions.push((y+i,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x+i]);
                if variant.starts_with("B"){
                    posible_actions.push((y+i,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x+i < 8 && y as i32 - i as i32 >= 0{
                if let Piece::Empty = board[y-i][x+i]{
                    posible_actions.push((y-i,x+i));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x+i]);
                if variant.starts_with("B"){
                    posible_actions.push((y-i,x+i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32 - i as i32 >= 0 && y + i < 8{
                if let Piece::Empty = board[y+i][x-i]{
                    posible_actions.push((y+i,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y+i][x-i]);
                if variant.starts_with("B"){
                    posible_actions.push((y+i,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
        'way: for i in 1..6 {
            if x as i32 - i as i32 >= 0 && y as i32 - i as i32 >=0{
                if let Piece::Empty = board[y-i][x-i]{
                    posible_actions.push((y-i,x-i));
                    continue;
                }
                let variant = format!("{:?}",board[y-i][x-i]);
                if variant.starts_with("B"){
                    posible_actions.push((y-i,x-i));
                    break 'way;
                }
                else {
                    break 'way;
                }
                
            }
        }
    }

    if let Piece::BKing = piece {
        let attacked_boxes = verify_attack("B", board, x, y);
        
        if x < 7 {
            if !(attacked_boxes.contains(&(y,x+1))) {
                let variant = format!("{:?}",&board[y][x+1]);
                if !(variant.starts_with("B")) {
                    posible_actions.push((y,x+1));
                }
            }
        }

        if x > 0 {
            if !(attacked_boxes.contains(&(y,x-1))) {
                let variant = format!("{:?}",&board[y][x-1]);
                if !(variant.starts_with("B")) {
                    posible_actions.push((y,x-1));
                }
            }
        }
        if x < 7 && y < 7 {
            if !(attacked_boxes.contains(&(y+1,x+1))) {
                let variant = format!("{:?}",&board[y+1][x+1]);
                if !(variant.starts_with("B")) {
                    posible_actions.push((y+1,x+1));
                }
            }
        }
        if x < 7 && y > 0 {
            if !(attacked_boxes.contains(&(y-1,x+1))) {
                let variant = format!("{:?}",&board[y-1][x+1]);
                if !(variant.starts_with("B")) {
                    posible_actions.push((y-1,x+1));
                }
            }
        }
        if y > 0 {
            if !(attacked_boxes.contains(&(y-1,x))) {
                let variant = format!("{:?}",&board[y-1][x]);
                if !(variant.starts_with("B")) {
                    posible_actions.push((y-1,x));
                }
            }
        }
        if y < 7 {
            if !(attacked_boxes.contains(&(y+1,x))) {
                let variant = format!("{:?}",&board[y+1][x]);
                if !(variant.starts_with("B")) {
                    posible_actions.push((y+1,x));
                }
            }
        }

        if x > 0 && y < 7 {
            if !(attacked_boxes.contains(&(y+1,x-1))) {
                let variant = format!("{:?}",&board[y+1][x-1]);
                if !(variant.starts_with("B")) {
                    posible_actions.push((y+1,x-1));
                }
            }
        }
        if x < 7 && y < 7 {
            if !(attacked_boxes.contains(&(y+1,x+1))) {
                let variant = format!("{:?}",&board[y+1][x+1]);
                if !(variant.starts_with("B")) {
                    posible_actions.push((y+1,x+1));
                }
            }
        }
    }

    if let Piece::WKing = piece {
        let attacked_boxes = verify_attack("W", board, x, y);
        
        if x < 7 {
            if !(attacked_boxes.contains(&(y,x+1))) {
                let variant = format!("{:?}",&board[y][x+1]);
                if !(variant.starts_with("W")) {
                    posible_actions.push((y,x+1));
                }
            }
        }

        if x > 0 {
            if !(attacked_boxes.contains(&(y,x-1))) {
                let variant = format!("{:?}",&board[y][x-1]);
                if !(variant.starts_with("W")) {
                    posible_actions.push((y,x-1));
                }
            }
        }
        if x < 7 && y < 7 {
            if !(attacked_boxes.contains(&(y+1,x+1))) {
                let variant = format!("{:?}",&board[y+1][x+1]);
                if !(variant.starts_with("W")) {
                    posible_actions.push((y+1,x+1));
                }
            }
        }
        if x < 7 && y > 0 {
            if !(attacked_boxes.contains(&(y-1,x+1))) {
                let variant = format!("{:?}",&board[y-1][x+1]);
                if !(variant.starts_with("W")) {
                    posible_actions.push((y-1,x+1));
                }
            }
        }
        if y > 0 {
            if !(attacked_boxes.contains(&(y-1,x))) {
                let variant = format!("{:?}",&board[y-1][x]);
                if !(variant.starts_with("W")) {
                    posible_actions.push((y-1,x));
                }
            }
        }
        if y < 7 {
            if !(attacked_boxes.contains(&(y+1,x))) {
                let variant = format!("{:?}",&board[y+1][x]);
                if !(variant.starts_with("W")) {
                    posible_actions.push((y+1,x));
                }
            }
        }

        if x > 0 && y < 7 {
            if !(attacked_boxes.contains(&(y+1,x-1))) {
                let variant = format!("{:?}",&board[y+1][x-1]);
                if !(variant.starts_with("W")) {
                    posible_actions.push((y+1,x-1));
                }
            }
        }
        if x < 7 && y < 7 {
            if !(attacked_boxes.contains(&(y+1,x+1))) {
                let variant = format!("{:?}",&board[y+1][x+1]);
                if !(variant.starts_with("W")) {
                    posible_actions.push((y+1,x+1));
                }
            }
        }
    }
    
    dbg!(&posible_actions);
    if posible_actions.contains(&where_to_move) {
        true
    }
    else {
        false
    }
}

fn verify_attack(color: &str, board: &Vec<Vec<Piece>>,x: usize, y : usize) -> Vec<(usize,usize)> {
    let mut posible_actions = vec![];

    if color == "B" {
        for row in board {
            for piece in row {

                if let Piece::WPawn = piece {
                    if x > 0 && y > 0{
                        posible_actions.push((y-1,x-1));
                    }
                    if x < 7 && y > 0 {
                        posible_actions.push((y-1,x+1));
                    }
                }

                if let Piece::WBishop = piece{
                    'way: for i in 1..6 {
                        if x+i < 8 && y+i < 8{
                            if let Piece::Empty = board[y+i][x+i]{
                                posible_actions.push((y+i,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x+i]);
                            if variant.starts_with("B") || variant.starts_with("w"){
                                posible_actions.push((y+i,x+i));
                                break 'way;
                            }
                            
                        }
                    }
                    'way: for i in 1..6 {
                        if x+i < 8 && y as i32 - i as i32 >= 0{
                            if let Piece::Empty = board[y-i][x+i]{
                                posible_actions.push((y-i,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x+i]);
                            if variant.starts_with("B") || variant.starts_with("w"){
                                posible_actions.push((y-i,x+i));
                                break 'way;
                            }
                            
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32 - i as i32 >= 0 && y + i < 8{
                            if let Piece::Empty = board[y+i][x-i]{
                                posible_actions.push((y+i,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x-i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y+i,x-i));
                                break 'way;
                            }
                            
                            
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32 - i as i32 >= 0 && y as i32 - i as i32 >=0{
                            if let Piece::Empty = board[y-i][x-i]{
                                posible_actions.push((y-i,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x-i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y-i,x-i));
                                break 'way;
                            }
                            
                            
                        }
                    }
                }

                if let Piece::WRook = piece{
                    'way: for i in 1..6{
                        if x+i < 8{
                            if let Piece::Empty = board[y][x+i]{
                                posible_actions.push((y,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y][x+i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y,x+i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6{
                        if y as i32 - i as i32 >= 0{
                            if let Piece::Empty = board[y-i][x]{
                                posible_actions.push((y-i,x));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y-i,x));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32-i as i32 >= 0{
                            if let Piece::Empty = board[y][x-i]{
                                posible_actions.push((y,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y][x-i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y,x-i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if y+i < 8{
                            if let Piece::Empty = board[y+i][x]{
                                posible_actions.push((y+i,x));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y+i,x));
                                break 'way;
                            }
                        }
                    }
                }   

                if let Piece::WQueen = piece {
                    'way: for i in 1..6{
                        if x+i < 8{
                            if let Piece::Empty = board[y][x+i]{
                                posible_actions.push((y,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y][x+i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y,x+i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6{
                        if y as i32 - i as i32 >= 0{
                            if let Piece::Empty = board[y-i][x]{
                                posible_actions.push((y-i,x));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y-i,x));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32-i as i32 >= 0{
                            if let Piece::Empty = board[y][x-i]{
                                posible_actions.push((y,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y][x-i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y,x-i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if y+i < 8{
                            if let Piece::Empty = board[y+i][x]{
                                posible_actions.push((y+i,x));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y+i,x));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x+i < 8 && y+i < 8{
                            if let Piece::Empty = board[y+i][x+i]{
                                posible_actions.push((y+i,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x+i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y+i,x+i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x+i < 8 && y as i32 - i as i32 >= 0{
                            if let Piece::Empty = board[y-i][x+i]{
                                posible_actions.push((y-i,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x+i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y-i,x+i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32 - i as i32 >= 0 && y + i < 8{
                            if let Piece::Empty = board[y+i][x-i]{
                                posible_actions.push((y+i,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x-i]);
                        
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y+i,x-i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32 - i as i32 >= 0 && y as i32 - i as i32 >=0{
                            if let Piece::Empty = board[y-i][x-i]{
                                posible_actions.push((y-i,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x-i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y-i,x-i));
                                break 'way;
                            }
                        }
                    }
                }

                if let Piece::WHorse = piece {
                    if x+1 < 8 && y + 2 < 8{
                        posible_actions.push((y+2,x+1));
                    }
                    if x as i32 - 1 >= 0 && y +  2 < 8 {
                        posible_actions.push((y+2,x-1));
                    }
                    if x+1 < 8 && y as i32 - 2 >= 0 {
                        posible_actions.push((y-2,x+1));
                    }    
                    if x as i32 - 1 >= 0 && y as i32 - 2 >= 0 {
                        posible_actions.push((y-2,x-1));
                    }
                    if y+1 < 8 && x + 2 < 8 {
                        posible_actions.push((y+1,x+2));
                    }
                    if y as i32 - 1 >= 0 && x + 2 < 8 {
                        posible_actions.push((y-1,x+2));
                    }
                    if y+1 < 8 && x as i32 - 2 >= 0 {
                        posible_actions.push((y+1,x-2));
                    }
                    if y as i32 - 1 >= 0 && x as i32 - 2 >= 0 {
                        posible_actions.push((y-1,x-2));
                    }
                }

            }
        }
    }

    else {
        for row in board {
            for piece in row {

                if let Piece::BPawn = piece {
                    if x > 0 && y < 7{
                        posible_actions.push((y+1,x-1));
                    }
                    if x < 7 && y > 0 {
                        posible_actions.push((y+1,x+1));
                    }
                }

                if let Piece::BBishop = piece{
                    'way: for i in 1..6 {
                        if x+i < 8 && y+i < 8{
                            if let Piece::Empty = board[y+i][x+i]{
                                posible_actions.push((y+i,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x+i]);
                            if variant.starts_with("B") || variant.starts_with("w"){
                                posible_actions.push((y+i,x+i));
                                break 'way;
                            }
                            
                        }
                    }
                    'way: for i in 1..6 {
                        if x+i < 8 && y as i32 - i as i32 >= 0{
                            if let Piece::Empty = board[y-i][x+i]{
                                posible_actions.push((y-i,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x+i]);
                            if variant.starts_with("B") || variant.starts_with("w"){
                                posible_actions.push((y-i,x+i));
                                break 'way;
                            }
                            
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32 - i as i32 >= 0 && y + i < 8{
                            if let Piece::Empty = board[y+i][x-i]{
                                posible_actions.push((y+i,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x-i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y+i,x-i));
                                break 'way;
                            }
                            
                            
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32 - i as i32 >= 0 && y as i32 - i as i32 >=0{
                            if let Piece::Empty = board[y-i][x-i]{
                                posible_actions.push((y-i,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x-i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y-i,x-i));
                                break 'way;
                            }
                            
                            
                        }
                    }
                }

                if let Piece::BRook = piece{
                    'way: for i in 1..6{
                        if x+i < 8{
                            if let Piece::Empty = board[y][x+i]{
                                posible_actions.push((y,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y][x+i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y,x+i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6{
                        if y as i32 - i as i32 >= 0{
                            if let Piece::Empty = board[y-i][x]{
                                posible_actions.push((y-i,x));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y-i,x));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32-i as i32 >= 0{
                            if let Piece::Empty = board[y][x-i]{
                                posible_actions.push((y,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y][x-i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y,x-i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if y+i < 8{
                            if let Piece::Empty = board[y+i][x]{
                                posible_actions.push((y+i,x));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y+i,x));
                                break 'way;
                            }
                        }
                    }
                }   

                if let Piece::BQueen = piece {
                    'way: for i in 1..6{
                        if x+i < 8{
                            if let Piece::Empty = board[y][x+i]{
                                posible_actions.push((y,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y][x+i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y,x+i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6{
                        if y as i32 - i as i32 >= 0{
                            if let Piece::Empty = board[y-i][x]{
                                posible_actions.push((y-i,x));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y-i,x));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32-i as i32 >= 0{
                            if let Piece::Empty = board[y][x-i]{
                                posible_actions.push((y,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y][x-i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y,x-i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if y+i < 8{
                            if let Piece::Empty = board[y+i][x]{
                                posible_actions.push((y+i,x));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y+i,x));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x+i < 8 && y+i < 8{
                            if let Piece::Empty = board[y+i][x+i]{
                                posible_actions.push((y+i,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x+i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y+i,x+i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x+i < 8 && y as i32 - i as i32 >= 0{
                            if let Piece::Empty = board[y-i][x+i]{
                                posible_actions.push((y-i,x+i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x+i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y-i,x+i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32 - i as i32 >= 0 && y + i < 8{
                            if let Piece::Empty = board[y+i][x-i]{
                                posible_actions.push((y+i,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y+i][x-i]);
                        
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y+i,x-i));
                                break 'way;
                            }
                        }
                    }
                    'way: for i in 1..6 {
                        if x as i32 - i as i32 >= 0 && y as i32 - i as i32 >=0{
                            if let Piece::Empty = board[y-i][x-i]{
                                posible_actions.push((y-i,x-i));
                                continue;
                            }
                            let variant = format!("{:?}",board[y-i][x-i]);
                            if variant.starts_with("B") || variant.starts_with("W"){
                                posible_actions.push((y-i,x-i));
                                break 'way;
                            }
                        }
                    }
                }

                if let Piece::BHorse = piece {
                    if x+1 < 8 && y + 2 < 8{
                        posible_actions.push((y+2,x+1));
                    }
                    if x as i32 - 1 >= 0 && y +  2 < 8 {
                        posible_actions.push((y+2,x-1));
                    }
                    if x+1 < 8 && y as i32 - 2 >= 0 {
                        posible_actions.push((y-2,x+1));
                    }    
                    if x as i32 - 1 >= 0 && y as i32 - 2 >= 0 {
                        posible_actions.push((y-2,x-1));
                    }
                    if y+1 < 8 && x + 2 < 8 {
                        posible_actions.push((y+1,x+2));
                    }
                    if y as i32 - 1 >= 0 && x + 2 < 8 {
                        posible_actions.push((y-1,x+2));
                    }
                    if y+1 < 8 && x as i32 - 2 >= 0 {
                        posible_actions.push((y+1,x-2));
                    }
                    if y as i32 - 1 >= 0 && x as i32 - 2 >= 0 {
                        posible_actions.push((y-1,x-2));
                    }
                }

            }
        }
    }

    let attacked_boxes = posible_actions;
    attacked_boxes
}

fn move_piece(board: &mut Vec<Vec<Piece>>,piece: (usize, usize), where_to_move: (usize,usize)){
    let x_now = piece.1;
    let y_now = piece.0;
    let x_after = where_to_move.1;
    let y_after = where_to_move.0;
    if verify(piece, where_to_move, board) {
        let variant = board[y_now][x_now].clone();
        board[y_now][x_now] = Piece::Empty;
        board[y_after][x_after] = variant;
    }
}
