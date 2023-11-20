mod char;
mod usize;

pub trait ToEnigmaInt<C, U> {
    fn to_internal_int(self) -> U;
}

pub trait FromEnigmaInt<U, C> {
    fn from_internal_int(self) -> C;
}
