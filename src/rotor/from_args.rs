use crate::enigma_int::ToEnigmaInt;

use super::Rotor;

impl Rotor {
    pub fn from_args(encoding: &String, turnover: &String, position: &String) -> Rotor {
        // Basic Checks
        if encoding.len() != 26 {
            panic!("Rotor encoding must be 26 characters long.\n --rotor \"ENCODING\" TURNOVER POSITION");
        }

        if turnover.len() != 1 {
            panic!("Rotor turnover must be 1 character.\n --rotor \"ENCODING\" TURNOVER POSITION");
        }

        if position.len() != 1 {
            panic!("Rotor position must be 1 character.\n --rotor \"ENCODING\" TURNOVER POSITION");
        }

        let wiring = encoding
            .chars()
            .map(|c| c.to_internal_int())
            .collect::<Vec<usize>>();

        // Validate turnover
        let turnover = turnover.chars().next().unwrap().to_internal_int();

        let turnover_index = match wiring.iter().position(|&c| c == turnover) {
            Some(i) => i,
            None => panic!("Rotor turnover must be a character of the encoding."),
        };

        // Validate position
        let position = position.chars().next().unwrap().to_internal_int();

        let position_index = match wiring.iter().position(|&c| c == position) {
            Some(i) => i,
            None => panic!("Rotor position must be a character of the encoding"),
        };

        Rotor {
            wiring,
            turnover: turnover_index,
            position: position_index,
        }
    }
}
