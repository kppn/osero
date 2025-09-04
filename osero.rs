use std::fmt;
use std::result::Result;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Piece {
    Black,
    White,
    Blank
}

impl Piece {
    fn is_blank(&self) -> bool {
        if let Self::Blank = self {
            true
        }
        else {
            false
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece_image = match self {
            Piece::Black => '*',
            Piece::White => 'o',
            Piece::Blank => '-',
        };
        write!(f, "{}", piece_image)
    }
}

struct Board {
    spaces: Vec<Vec<Piece>>
}

impl Board {
    fn new() -> Board {
        let mut xv: Vec<Vec<Piece>> = Vec::new();
        for _ in 0..8 {
            let mut yv: Vec<Piece> = Vec::new();
            for _ in 0..8 {
                yv.push(Piece::Blank);
            }
            xv.push(yv);
        }
        xv[3][3] = Piece::Black;
        xv[4][4] = Piece::Black;
        xv[3][4] = Piece::White;
        xv[4][3] = Piece::White;
        Board { spaces: xv }
    }

    fn print(&self) {
        println!("  0 1 2 3 4 5 6 7");
        for _x in 0..8 {
            print!("{} ", _x);
            for _y in 0..8 {
                print!("{} ", &self.spaces[_x][_y]);
            }
            println!("");
        }

        let (n_black, n_white) = self.count();
        println!("Black: {}, White: {}", n_black, n_white);
    }

    fn count(&self) -> (usize, usize) {
        let flat_spaces: Vec<&Piece> = self.spaces.iter().flatten().collect();

        // クロージャを返す関数。数える色(target: Black/White)をキャプチャ
        fn counter<'a>(target: Piece) -> impl Fn(usize, &&'a Piece) -> usize {
            move |sum, &piece| if *piece == target { sum + 1 } else { sum }
        }

        let n_black = flat_spaces.iter().fold(0, counter(Piece::Black));
        let n_white = flat_spaces.iter().fold(0, counter(Piece::White));

        (n_black, n_white)
    }

    fn place(&mut self, x: usize, y: usize, piece: &Piece) -> Result<(), ()> {
        if ! Self::_is_placable(&self.spaces, x, y, piece) {
            return Result::Err(());
        }

        self.spaces[x][y] = *piece;
        Self::_reverse(&mut self.spaces, x, y, piece);

        Result::Ok(())
    }

    fn _is_placable(spaces: &Vec<Vec<Piece>>, x: usize, y: usize, piece: &Piece) -> bool {
        if ! spaces[x][y].is_blank() {
            return false;
        }
        if ! Self::_is_piece_around(spaces, x, y, piece) {
            return false;
        }
        true
    }

    fn _is_blank(spaces: &Vec<Vec<Piece>>, x: usize, y: usize) -> bool {
        if let Piece::Blank = spaces[x][y] {
            true
        }
        else {
            false
        }
    }

    fn _is_piece_around(spaces: &Vec<Vec<Piece>>, x: usize, y: usize, piece: &Piece) -> bool {
        let x_range = Self::_around_range(x);
        let y_range = Self::_around_range(y);

        for cx in x_range.clone() {
            for cy in y_range.clone() {
                if (x==cx) && (y==cy) {
                    continue;
                }
                if (!spaces[cx][cy].is_blank()) && *piece != spaces[cx][cy] {
                    return true;
                }
            }
        }
        false
    }

    fn _reverse(spaces: &mut Vec<Vec<Piece>>, x: usize, y: usize, piece: &Piece) {
        let fwd = |n: usize| n + 1;
        let bak = |n: usize| n.wrapping_sub(1);
        let blk = |n: usize| n + 0;
        Self::_reverse_recv(spaces, x+1,               y,                 &fwd, &blk, piece);
        Self::_reverse_recv(spaces, x+1,               y+1,               &fwd, &fwd, piece);
        Self::_reverse_recv(spaces, x,                 y+1,               &blk, &fwd, piece);
        Self::_reverse_recv(spaces, x.wrapping_sub(1), y+1,               &bak, &fwd, piece);
        Self::_reverse_recv(spaces, x.wrapping_sub(1), y,                 &bak, &blk, piece);
        Self::_reverse_recv(spaces, x.wrapping_sub(1), y.wrapping_sub(1), &bak, &bak, piece);
        Self::_reverse_recv(spaces, x,                 y.wrapping_sub(1), &blk, &bak, piece);
        Self::_reverse_recv(spaces, x+1,               y.wrapping_sub(1), &fwd, &bak, piece);
    }

    fn _reverse_recv(spaces: &mut Vec<Vec<Piece>>, x: usize, y: usize, x_next: &dyn Fn(usize) -> usize, y_next: &dyn Fn(usize) -> usize, piece: &Piece) -> bool
    {
        if (x==8) || (y==8) || (x==usize::MAX) || (y==usize::MAX) {
            return false;
        }
        if let Piece::Blank = spaces[x][y] {
            return false;
        }

        if *piece == spaces[x][y] {
            return true;
        }
        else {
            let do_reverse = Self::_reverse_recv(spaces, x_next(x), y_next(y), x_next, y_next, piece);
            if do_reverse {
                spaces[x][y] = *piece;
            }
            do_reverse
        }
    }

    fn _around_range(n: usize) -> std::ops::RangeInclusive<usize> {
        if n == 0 {
            0..=2
        }
        else if n == 7 {
            5..=7
        }
        else {
            n-1..=n+1
        }
    }

}



fn main() {
    println!("Wellcome Osero");
    println!("");

    let mut board = Board::new();
    board.print();

    let mut te = Piece::Black;

    loop {
        println!("Phase {}", te);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let x_y: Vec<&str> = input.trim().split(',').collect();
        if x_y.len() != 2 {
            println!("x,yで入力して");
            continue;
        }

        let x_y: Vec<usize> = input.trim().split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        let x = x_y[0];
        let y = x_y[1];

        if let Result::Ok(()) = board.place(x, y, &te) {
            if let Piece::Black = te {
                te = Piece::White;
            }
            else {
                te = Piece::Black;
            }
        }

        board.print();
    }
}



