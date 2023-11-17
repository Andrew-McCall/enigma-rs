pub trait ToEnigmaInt {
    fn to_internal_int(self) -> usize;
}

pub trait FromEnigmaInt {
    fn from_internal_int(self) -> char;
}

impl ToEnigmaInt for char {
    /// This mimics turning a key press into an internal wire - which the rotors/plugboard will cipher
    fn to_internal_int(self) -> usize {
        // Ugly but efficient
        match self {
            'A' | 'a' => 0,
            'B' | 'b' => 1,
            'C' | 'c' => 2,
            'D' | 'd' => 3,
            'E' | 'e' => 4,
            'F' | 'f' => 5,
            'G' | 'g' => 6,
            'H' | 'h' => 7,
            'I' | 'i' => 8,
            'J' | 'j' => 9,
            'K' | 'k' => 10,
            'L' | 'l' => 11,
            'M' | 'm' => 12,
            'N' | 'n' => 13,
            'O' | 'o' => 14,
            'P' | 'p' => 15,
            'Q' | 'q' => 16,
            'R' | 'r' => 17,
            'S' | 's' => 18,
            'T' | 't' => 19,
            'U' | 'u' => 20,
            'V' | 'v' => 21,
            'W' | 'w' => 22,
            'X' | 'x' => 23,
            'Y' | 'y' => 24,
            'Z' | 'z' => 25,
            _ => panic!("Invalid character: {}", self),
        }
    }
}

impl FromEnigmaInt for usize {
    fn from_internal_int(self) -> char {
        match self {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            8 => 'I',
            9 => 'J',
            10 => 'K',
            11 => 'L',
            12 => 'M',
            13 => 'N',
            14 => 'O',
            15 => 'P',
            16 => 'Q',
            17 => 'R',
            18 => 'S',
            19 => 'T',
            20 => 'U',
            21 => 'V',
            22 => 'W',
            23 => 'X',
            24 => 'Y',
            25 => 'Z',
            _ => panic!("Invalid integer: {}", self),
        }
    }
}
