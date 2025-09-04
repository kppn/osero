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

    fn place(&mut self, x: isize, y: isize, piece: &Piece) -> Result<(), ()> {
        if ! Self::_is_placable(&self.spaces, x, y, piece) {
            return Result::Err(());
        }

        let (xi, yi) = (x as usize, y as usize);

        // ひっくり返せる場所を集める
        let mut reverse_reserve = Vec::<(i32, i32)>::new();
        Self::_select_reversable(&mut self.spaces, x, y, piece, &mut reverse_reserve);
        // 無かったらやり直し
        if reverse_reserve.len() == 0 {
            self.spaces[xi][yi] = Piece::Blank;
            return Result::Err(());
        }
        // あったらひっくり返し、置く
        for (rx, ry) in reverse_reserve {
            self.spaces[rx as usize][ry as usize] = *piece;
        }
        self.spaces[xi][yi] = *piece;

        Result::Ok(())
    }

    fn _is_placable(spaces: &Vec<Vec<Piece>>, x: isize, y: isize, piece: &Piece) -> bool {
        let (xi, yi) = (x as usize, y as usize);
        if ! spaces[xi][yi].is_blank() {
            return false;
        }
        if ! Self::_is_piece_around(spaces, x, y, piece) {
            return false;
        }
        true
    }

    fn _is_blank(spaces: &Vec<Vec<Piece>>, x: isize, y: isize) -> bool {
        let (xi, yi) = (x as usize, y as usize);
        if let Piece::Blank = spaces[xi][yi] {
            true
        }
        else {
            false
        }
    }

    fn _is_piece_around(spaces: &Vec<Vec<Piece>>, x: isize, y: isize, piece: &Piece) -> bool {
        let x_range = Self::_around_range(x);
        let y_range = Self::_around_range(y);

        for cx in x_range.clone() {
            for cy in y_range.clone() {
                if (x==cx) && (y==cy) {
                    continue;
                }
                let (xi, yi) = (cx as usize, cy as usize);
                if (!spaces[xi][yi].is_blank()) && *piece != spaces[xi][yi] {
                    return true;
                }
            }
        }
        false
    }

    fn _select_reversable(
        spaces: &mut Vec<Vec<Piece>>, 
        x: isize, y: isize, 
        piece: &Piece, 
        reverse_reserve: &mut Vec<(i32, i32)>
    ) {
        // 次に進む方向を示すクロージャ
        let fwd = |n: isize| n + 1;
        let bak = |n: isize| n - 1;
        let nul = |n: isize| n + 0;
        // 8方向を探索し、ひっくり返せる場所を探す
        Self::_reverse_recv(spaces, fwd(x), nul(y), &fwd, &nul, piece, reverse_reserve);
        Self::_reverse_recv(spaces, fwd(x), fwd(y), &fwd, &fwd, piece, reverse_reserve);
        Self::_reverse_recv(spaces, nul(x), fwd(y), &nul, &fwd, piece, reverse_reserve);
        Self::_reverse_recv(spaces, bak(x), fwd(y), &bak, &fwd, piece, reverse_reserve);
        Self::_reverse_recv(spaces, bak(x), nul(y), &bak, &nul, piece, reverse_reserve);
        Self::_reverse_recv(spaces, bak(x), bak(y), &bak, &bak, piece, reverse_reserve);
        Self::_reverse_recv(spaces, nul(x), bak(y), &nul, &bak, piece, reverse_reserve);
        Self::_reverse_recv(spaces, fwd(x), bak(y), &fwd, &bak, piece, reverse_reserve);
    }

    fn _reverse_recv(
        spaces: &mut Vec<Vec<Piece>>, 
        x: isize, y: isize, 
        x_next: &dyn Fn(isize) -> isize, 
        y_next: &dyn Fn(isize) -> isize,
        piece: &Piece,
        reverse_reserve: &mut Vec<(i32, i32)>
    ) -> bool
    {
        if (x>=8) || (y>=8) || (x<0) || (y<0) {
            return false;
        }

        let (xi, yi) = (x as usize, y as usize);

        if let Piece::Blank = spaces[xi][yi] {
            return false;
        }

        if *piece == spaces[xi][yi] {
            return true;
        }
        else {
            let do_reverse = Self::_reverse_recv(spaces, x_next(x), y_next(y), x_next, y_next, piece, reverse_reserve);
            if do_reverse {
                reverse_reserve.push((xi as i32, yi as i32));
                //spaces[xi][yi] = *piece;
            }
            do_reverse
        }
    }

    fn _around_range(n: isize) -> std::ops::RangeInclusive<isize> {
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
        let x_y: Vec<isize> = input.trim().split(',').map(|s| s.parse::<isize>().unwrap()).collect();
        let x = x_y[0];
        let y = x_y[1];
        if (x<0) || (x>7) || (y<0) || (y>7) {
            println!("0~7で入力して");
            continue;
        }

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



