mod usize;

pub trait ToEnigmaInt {
    fn to_internal_int(self) -> usize;
}

pub trait FromEnigmaInt {
    fn from_internal_int(self) -> char;
}
