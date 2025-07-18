mod enigma;
mod enigma_int;
mod plugboard;
mod reflector;
mod rotor;

pub use enigma::Enigma;
pub use reflector::Reflector;
pub use rotor::Rotor;

use crate::plugboard::Plugboard;

pub struct EnigmaBuilder {
    pub rotors: Vec<Rotor>,
    pub reflector: Option<Reflector>,
    pub plugboard: Option<Plugboard>,
}

impl EnigmaBuilder {
    pub fn new() -> EnigmaBuilder {
        EnigmaBuilder {
            rotors: Vec::new(),
            reflector: None,
            plugboard: None,
        }
    }
}

impl Default for EnigmaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EnigmaBuilder {
    pub fn build(self) -> Enigma {
        Enigma {
            rotors: self.rotors,
            reflector: self.reflector.expect("Reflector is required"),
            plugboard: self.plugboard,
        }
    }
}
