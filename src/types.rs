//! Core chess types: squares, colors, pieces, and castling rights.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
    Both = 2,
}

impl Color {
    #[inline(always)]
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::Both => Color::Both,
        }
    }
}

pub const SQUARES: [&str; 64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];

#[allow(non_upper_case_globals)]
pub mod square {
    pub const a8: usize = 0;  pub const b8: usize = 1;  pub const c8: usize = 2;  pub const d8: usize = 3;
    pub const e8: usize = 4;  pub const f8: usize = 5;  pub const g8: usize = 6;  pub const h8: usize = 7;
    pub const a7: usize = 8;  pub const b7: usize = 9;  pub const c7: usize = 10; pub const d7: usize = 11;
    pub const e7: usize = 12; pub const f7: usize = 13; pub const g7: usize = 14; pub const h7: usize = 15;
    pub const a6: usize = 16; pub const b6: usize = 17; pub const c6: usize = 18; pub const d6: usize = 19;
    pub const e6: usize = 20; pub const f6: usize = 21; pub const g6: usize = 22; pub const h6: usize = 23;
    pub const a5: usize = 24; pub const b5: usize = 25; pub const c5: usize = 26; pub const d5: usize = 27;
    pub const e5: usize = 28; pub const f5: usize = 29; pub const g5: usize = 30; pub const h5: usize = 31;
    pub const a4: usize = 32; pub const b4: usize = 33; pub const c4: usize = 34; pub const d4: usize = 35;
    pub const e4: usize = 36; pub const f4: usize = 37; pub const g4: usize = 38; pub const h4: usize = 39;
    pub const a3: usize = 40; pub const b3: usize = 41; pub const c3: usize = 42; pub const d3: usize = 43;
    pub const e3: usize = 44; pub const f3: usize = 45; pub const g3: usize = 46; pub const h3: usize = 47;
    pub const a2: usize = 48; pub const b2: usize = 49; pub const c2: usize = 50; pub const d2: usize = 51;
    pub const e2: usize = 52; pub const f2: usize = 53; pub const g2: usize = 54; pub const h2: usize = 55;
    pub const a1: usize = 56; pub const b1: usize = 57; pub const c1: usize = 58; pub const d1: usize = 59;
    pub const e1: usize = 60; pub const f1: usize = 61; pub const g1: usize = 62; pub const h1: usize = 63;
    pub const NO_SQ: usize = 64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Piece {
    P = 0,
    N = 1,
    B = 2,
    R = 3,
    Q = 4,
    K = 5,
    PawnBlack = 6,
    KnightBlack = 7,
    BishopBlack = 8,
    RookBlack = 9,
    QueenBlack = 10,
    KingBlack = 11,
}

pub const PIECE_CHARS: [char; 12] = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];

impl Piece {
    #[inline(always)]
    pub fn color(self) -> Color {
        if (self as u8) < 6 {
            Color::White
        } else {
            Color::Black
        }
    }

    #[inline(always)]
    pub fn to_char(self) -> char {
        PIECE_CHARS[self as usize]
    }

    pub fn from_char(c: char) -> Option<Piece> {
        match c {
            'P' => Some(Piece::P),
            'N' => Some(Piece::N),
            'B' => Some(Piece::B),
            'R' => Some(Piece::R),
            'Q' => Some(Piece::Q),
            'K' => Some(Piece::K),
            'p' => Some(Piece::PawnBlack),
            'n' => Some(Piece::KnightBlack),
            'b' => Some(Piece::BishopBlack),
            'r' => Some(Piece::RookBlack),
            'q' => Some(Piece::QueenBlack),
            'k' => Some(Piece::KingBlack),
            _ => None,
        }
    }
}

pub const CASTLE_WK: u8 = 1;
pub const CASTLE_WQ: u8 = 2;
pub const CASTLE_BK: u8 = 4;
pub const CASTLE_BQ: u8 = 8;

pub fn square_to_str(sq: usize) -> &'static str {
    if sq < 64 {
        SQUARES[sq]
    } else {
        "-"
    }
}

pub fn parse_square(s: &str) -> Option<usize> {
    if s.len() < 2 {
        return None;
    }
    let bytes = s.as_bytes();
    if bytes[0] < b'a' || bytes[0] > b'h' || bytes[1] < b'1' || bytes[1] > b'8' {
        return None;
    }
    let file = (bytes[0] - b'a') as usize;
    let rank = (8 - (bytes[1] - b'0')) as usize;
    Some(rank * 8 + file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_parsing_and_str() {
        assert_eq!(square_to_str(square::a8), "a8");
        assert_eq!(square_to_str(square::h1), "h1");
        assert_eq!(square_to_str(square::e4), "e4");
        assert_eq!(parse_square("a8"), Some(square::a8));
        assert_eq!(parse_square("h1"), Some(square::h1));
        assert_eq!(parse_square("e4"), Some(square::e4));
        assert_eq!(parse_square("e1"), Some(square::e1));
        assert_eq!(parse_square("e8"), Some(square::e8));
        assert_eq!(parse_square("xx"), None);
    }

    #[test]
    fn test_piece_conversions() {
        assert_eq!(Piece::P.to_char(), 'P');
        assert_eq!(Piece::KingBlack.to_char(), 'k');
        assert_eq!(Piece::from_char('Q'), Some(Piece::Q));
        assert_eq!(Piece::from_char('q'), Some(Piece::QueenBlack));
        assert_eq!(Piece::P.color(), Color::White);
        assert_eq!(Piece::KnightBlack.color(), Color::Black);
    }

    #[test]
    fn test_color_opposite() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(Color::Black.opposite(), Color::White);
    }
}
