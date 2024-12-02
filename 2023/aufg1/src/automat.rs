pub enum NumberDigitizeAutomat {
    S,
    A,
    B,
    C,
    O(u8),
}

impl NumberDigitizeAutomat {
    pub fn next(&self, char: &str) -> Self {
        match (self, char) {
            (Self::S, "z") => Self::O(0),
            (Self::S, "o") => Self::O(1),
            (Self::S, "t") => Self::A,
            (Self::S, "f") => Self::B,
            (Self::S, "s") => Self::C,
            (Self::S, "e") => Self::O(8),
            (Self::S, "n") => Self::O(9),
            (Self::A, "w") => Self::O(2),
            (Self::A, "h") => Self::O(3),
            (Self::A, "e") => Self::O(10),
            (Self::B, "o") => Self::O(4),
            (Self::B, "i") => Self::O(5),
            (Self::C, "i") => Self::O(6),
            (Self::C, "e") => Self::O(7),
            (Self::A, _) => Self::A,
            (Self::B, _) => Self::B,
            (Self::C, _) => Self::C,
            (Self::O(a), _) => Self::O(*a),
        }
    }
}
