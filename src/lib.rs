use std::{ops::{Neg, Index, IndexMut}, fmt::Display, io::Stdin};

pub const DIRS: [MoveDir; 4] = [MoveDir::TopLeft, MoveDir::TopRight, MoveDir::DownLeft, MoveDir::DownRight];

pub const LOST: i64 = -1_000_000;
pub const WIN: i64 = 1_000_000;


#[derive(Clone, PartialEq, Eq, Copy, Default)]
pub enum Color {
    White,
    #[default]
    Black
}

impl Color {
    pub fn from_str(s: &str) -> Self {
        if s=="white" {
            Self::White
        }
        else {
            assert!(s=="black");
            Self::Black
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "white"),
            Color::Black => write!(f, "black")
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Piece {
    pub king: bool,
    pub color: Color
}

impl Neg for Color {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}

#[derive(Default, Clone)]
pub struct Cell {
    pub piece: Option<Piece>
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CellPos {
    pub col: usize,
    pub row: usize
}

impl Display for CellPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = ('A' as u8 + self.col as u8) as char;
        write!(f, "{}{}", letter, self.row)
    }
}

impl CellPos {
    pub fn from_str(s: &str) -> Self {
        let mut ch = s.chars();

        let l = ch.next().unwrap() as u8 - 'A' as u8;
        let n = ch.next().unwrap() as u8 - '0' as u8; 
        Self {
            col: l as usize,
            row: n as usize
        } 
    }
    pub fn shift(self, mv_dir: MoveDir) -> Option<CellPos> {
        match mv_dir {
            MoveDir::DownLeft => {
                if self.col == 0 || self.row == 0 {
                    return None
                }
                Some(Self {col: self.col-1, row: self.row-1})
            }
            MoveDir::DownRight => {
                if self.row == 0 || self.col == 7 {
                    return None;
                }
                Some(Self {col: self.col+1, row: self.row-1})
            }
            MoveDir::TopLeft => {
                if self.row == 7 || self.col == 0 {
                    return None;
                }
                Some(Self {col: self.col-1, row: self.row+1})
            }
            MoveDir::TopRight => {
                if self.row == 7 || self.col == 7 {
                    return None;
                }
                Some(Self {col: self.col+1, row: self.row+1})
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MoveDir {
    TopRight,
    TopLeft,
    DownRight,
    DownLeft 
}

impl MoveDir {
    pub fn from_str(s: &str) -> Self {
        match s {
            "tr" => MoveDir::TopRight,
            "tl" => MoveDir::TopLeft,
            "dr" => MoveDir::DownRight,
            "dl" => MoveDir::DownLeft,
            _ => panic!("Invalid move direction format.")
        }
    }
}

impl Display for MoveDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveDir::TopRight => write!(f, "tr"),
            MoveDir::TopLeft => write!(f, "tl"),
            MoveDir::DownRight => write!(f, "dr"),
            MoveDir::DownLeft => write!(f, "dl")
        }
    }
}

#[derive(Clone, Copy)]
pub struct PieceMove {
    pub pos: CellPos, 
    pub dir: MoveDir
}

impl Display for PieceMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.pos, self.dir)
    }
}

impl PieceMove {
    pub fn from_stdin(stdin: &mut Stdin) -> Self {
        let mut content = String::new();
        stdin.read_line(&mut content).unwrap();
        content = content.trim().to_string();
        
        Self::from_str(&content)
    }
    pub fn from_str(s: &str) -> Self {
        let mut sp = s.split_whitespace();
        let pos = CellPos::from_str(sp.next().unwrap());
        let dir = MoveDir::from_str(sp.next().unwrap());

        Self {
            pos, dir
        }
    }
}

#[derive(Default, Clone)]
pub struct Board {
    pub must_jump: Vec<CellPos>,
    pub data: [[Cell; 8]; 8],
    pub turn: Color,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("   ");
        for i in 0..8 {
            let letter = ('A' as u8 + i as u8) as char;
            s.push(letter);
            s.push(' ');
        }
        s.push('\n');
        for (i, row) in self.data.iter().enumerate().rev() {
            s.push_str(format!("{}: ", i).as_str());
            for cell in row.iter() {
                let ch = match cell.piece {
                    Some(Piece {king: false, color: Color::White}) => "w",
                    Some(Piece {king: false, color: Color::Black}) => "b",
                    Some(Piece {king: true, color: Color::White}) => "W",
                    Some(Piece {king: true, color: Color::Black}) => "B",
                    None => "."
                };
                s.push_str(ch);
                s.push(' ');
            }
            s.push('\n');
        }
        s.pop();
        write!(f, "{}", s)
    }
}

impl Index<CellPos> for Board {
    type Output = Cell;
    fn index(&self, index: CellPos) -> &Self::Output {
        &self.data[index.row][index.col]
    }    
}

impl IndexMut<CellPos> for Board {
    fn index_mut(&mut self, index: CellPos) -> &mut Self::Output {
        &mut self.data[index.row][index.col]
    }
}

impl Board {
    pub fn new() -> Self {
        let mut data: [[Cell; 8]; 8] = Default::default();
        for i in 0..8 {
            for j in 0..8 {
                if (i+j)%2 == 1 {
                    continue;
                }
                let piece = if i < 3 {
                    Some(Piece {king: false, color: Color::White})
                }
                else if i > 4 {
                    Some(Piece {king: false, color: Color::Black})
                }
                else {
                    None
                };
                data[i][j].piece = piece;
            }
        }
        Self {
            data,
            ..Default::default()
        }
    }

    pub fn exists_valid_move(&self) -> bool {
        let mut board = self.clone();
        for i in 0..8 {
            for j in 0..8 {
                for dir in DIRS {
                    if board.make_move(PieceMove { pos: CellPos {col: j, row: i}, dir }) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn make_move(&mut self, mv: PieceMove) -> bool {
        let Some(cpiece) = self[mv.pos].piece else {
            return false;
        };
        if cpiece.color != self.turn {
            return false;
        }

        if self.must_jump.len() != 0 && !self.must_jump.contains(&mv.pos) {
            return false;
        }

        if !cpiece.king {
            if cpiece.color == Color::White {
                if mv.dir == MoveDir::DownLeft || mv.dir == MoveDir::DownRight {
                    return false;
                }
            }
            else {
                if mv.dir == MoveDir::TopLeft || mv.dir == MoveDir::TopRight {
                    return false;
                }
            }
        }

        let Some(npos) = mv.pos.shift(mv.dir) else {
            return false;
        };

        if let Some(npiece) = self[npos].piece {
            if npiece.color == self.turn {
                return false;
            }
            let Some(nnpos) = npos.shift(mv.dir) else {
                return false;
            };
            if self[nnpos].piece.is_some() {
                return false;
            }
            self[nnpos].piece = Some(cpiece);
            self[mv.pos].piece = None;
            self[npos].piece = None;
            if self.can_jump(nnpos) {
                self.must_jump = vec![nnpos];
            }
            else {
                self.turn = -self.turn;
                self.find_forced_jumps();
            }
        }
        else {
            if self.must_jump.len() != 0 {
                return false;
            }
            self[mv.pos].piece = None;
            self[npos].piece = Some(cpiece);
            self.turn = -self.turn;
            self.find_forced_jumps();
        }
        self.promote_pawns();

        true
    }

    fn find_forced_jumps(&mut self) {
        self.must_jump = vec![];
        let mut k = false;
        for (i, row) in self.data.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let Some(piece) = cell.piece else {
                    continue;
                };
                let cp = CellPos {row: i, col: j};
                if piece.color == self.turn && self.can_jump(cp) {
                    if piece.king && !k  {
                        self.must_jump = vec![];
                        k = true;
                    }
                    if piece.king || !k {
                        self.must_jump.push(cp);
                    }
                }
            }
        }
    }

    pub fn can_jump(&self, cp: CellPos) -> bool {
        let Some(piece) = self[cp].piece else {
            panic!("Can jump should be called on a square with a piece.");
        };

        assert!(piece.color == self.turn);

        let mut dirs_to_check = vec![]; 
        if piece.king || piece.color == Color::White {
            dirs_to_check.push(MoveDir::TopRight);
            dirs_to_check.push(MoveDir::TopLeft);
        }
        if piece.king || piece.color == Color::Black {
            dirs_to_check.push(MoveDir::DownLeft);
            dirs_to_check.push(MoveDir::DownRight);
        }

        for dir in dirs_to_check {
            let Some(np) = cp.shift(dir) else {
                continue;
            };
            let Some(npiece) = self[np].piece else {
                continue;
            };
            if npiece.color != -self.turn {
                continue;
            }
            let Some(nnp) = np.shift(dir) else {
                continue;
            };

            if self[nnp].piece.is_none() {
                return true;
            }
        }

        return false;
        
    }

    pub fn from_stdin(stdin: &Stdin) -> Self {
        let mut data: [[Cell; 8]; 8] = Default::default();
        let content = (0..9).into_iter().map(|_| {
            let mut s = String::new();
            stdin.read_line(&mut s).unwrap();
            s.trim().split_whitespace().skip(1).map(|s| s.to_string()).collect::<Vec<_>>()
        }).skip(1).collect::<Vec<_>>();
        assert!(content.len() == 8);
        for (row, row_data) in content.into_iter().enumerate() {
            // eprintln!("{:?}", row_data);
            assert!(row_data.len() == 8);
            for (col, ch) in row_data.into_iter().enumerate() {
                assert!(ch.len() == 1);
                let piece = match ch.as_str() {
                    "w" => Some(Piece {king: false, color: Color::White}),
                    "b" => Some(Piece {king: false, color: Color::Black}),
                    "W" => Some(Piece {king: true, color: Color::White}),
                    "B" => Some(Piece {king: true, color: Color::Black}),
                    _ => { assert!(ch == "."); None }
                };
                data[8-row-1][col].piece = piece;
            }
        }

        let mut result = Self {
            data,
            ..Default::default()
        };
        result.promote_pawns();
        result.find_forced_jumps();
        result
    }

    fn promote_pawns(&mut self) {
        for i in 0..8 {
            let tcp = CellPos {row: 7, col: i};
            if let Some(piece) = &mut self[tcp].piece {
                if piece.color == Color::White {
                    piece.king = true;
                }
            }
            let bcp = CellPos {row: 0, col: i};
            if let Some(piece) = &mut self[bcp].piece {
                if piece.color == Color::Black {
                    piece.king = true;
                }
            }
        }
    }

    pub fn piece_pos(&self, color: Color) -> Vec<CellPos> {
        let mut result = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let cp = CellPos {row: i, col: j};
                let Some(piece) = self[cp].piece else {
                    continue;
                };
                if piece.color == color {
                    result.push(cp);
                }
            }
        }
        result
    }
}


pub fn heuristic(board: &Board) -> i64 {
    let mut res = 0;

    let row_vals_pawn = [7, 0, 1, 2, 3, 4, 5, 9];
    let row_vals_king = [1, 2, 2, 3, 3, 2, 2, 1];
    let sq_6x6_val = 3;
    let sq_4x4_val = 1;
    let piece_val = 5;
    let king_val = 10;  // Increase the king value, as it's often more valuable than a pawn.
    let mis_neighbor_val = -1;
    let exposed_pawn_val = -2;
    let trn_jump_val = 3;

    for color in [Color::White, Color::Black] {
        let mp = if board.turn == color {1} else {-1};
        let mut lres = 0;

        for cp in board.piece_pos(color) {
            let piece = board[cp].piece.unwrap();
            let rrow = if color == Color::White {cp.row} else { 8-cp.row-1 };

            lres += piece_val;

            if cp.col >= 1 && cp.col <= 6 && cp.row >= 1 && cp.row <= 6 {
                lres += sq_6x6_val;
                if cp.col >= 2 && cp.col <= 5 && cp.row >= 2 && cp.row <= 5 {
                    lres += sq_4x4_val;
                }
            }

            if color == board.turn {
                if board.can_jump(cp) {
                    lres += trn_jump_val;
                }
            }

            if piece.king {
                lres += king_val;
                lres += row_vals_king[rrow];
            }
            else {
                lres += row_vals_pawn[rrow];
                let (ls, rs) = if color == Color::White {(MoveDir::TopLeft, MoveDir::TopRight)} else {(MoveDir::DownLeft, MoveDir::DownRight)};
                let lnpos = cp.shift(ls);
                let rnpos = cp.shift(rs);
                let (b_ls, b_rs) = if color == Color::White {(MoveDir::DownLeft, MoveDir::DownRight)} else {(MoveDir::TopLeft, MoveDir::TopRight)};
    
                // Checking for exposed pawn in both backward directions
                for back_dir in [b_ls, b_rs] {
                    let back_pos = cp.shift(back_dir);
                    if let Some(back_pos) = back_pos {
                        if board[back_pos].piece.is_none() {
                            lres += exposed_pawn_val;
                        }
                    }
                }


                for npos in [lnpos, rnpos] {
                    if let Some(npos) = npos {
                        if board[npos].piece.is_none() {
                            lres -= mis_neighbor_val;
                        }
                    }
                }
            }

        }

        res += lres*mp;
    }
    res
}

pub fn sort_by_heuristic<T: Fn(&Board) -> i64>(mut board: Board, poss: Vec<CellPos>, h_fn: T) -> Vec<CellPos> {
    let old_board = board.clone();
    let mut poss = poss.into_iter().map(|cp| {
        let mut lbst = LOST;
        for dir in DIRS {
            let pm = PieceMove {pos: cp, dir};
            if board.make_move(pm) {
                let heur = heuristic(&board);
                let score = if old_board.turn == board.turn {heur} else {-heur};
                lbst = lbst.max(score);
                board = old_board.clone();
            }
        }
        (lbst, cp)
    }).collect::<Vec<_>>();

    poss.sort_by_key(|x| x.0);
    poss.reverse();
    poss.into_iter().map(|(_, cp)| cp).collect::<Vec<_>>()
}
