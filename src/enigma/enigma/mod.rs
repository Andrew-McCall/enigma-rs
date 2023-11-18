use crate::{reflector::Reflector, rotor::Rotor};

mod encode;

pub struct Enigma {
    pub rotors: Vec<Rotor>,
    pub reflector: Reflector,
}
