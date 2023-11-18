use crate::enigma_int::FromEnigmaInt;

use super::Rotor;

impl Rotor {
    pub fn get_position(&self) -> char {
        // Position is added during encoding ;)
        self.encode(0).from_internal_int()
    }
}
