use crate::enigma_int::ToEnigmaInt;

use super::Plugboard;

impl Plugboard {
    pub fn from_args(argument: &String) -> Plugboard {
        let pairs = argument.split_whitespace();

        let mut wiring = [0; 26];

        for pair in pairs.clone() {
            let mut chars = pair.chars();
            let first = chars.next().unwrap();
            let second = chars.next().unwrap();

            if !first.is_ascii_alphabetic() || !second.is_ascii_alphabetic() {
                panic!(
                    "{} is invalid. Plugboard pairs must be letters.\n --plugboard \"AB\"",
                    pair
                );
            }

            let first_int = first.to_internal_int();
            let second_int = second.to_internal_int();

            if wiring[first_int] != 0 || wiring[second_int] != 0 || first_int == second_int {
                panic!("Reflector pairs must not have duplicate letters.");
            }

            wiring[first_int] = second_int;
            wiring[second_int] = first_int;
        }

        // Fill in the rest of the wiring
        for i in 1..25 {
            if wiring[i] == 0 {
                wiring[i] = i;
            }
        }

        // if pairs.count() > 10 {
        //     println!("WARNING: Plugboard has more than 10 pairs. Not historically accurate.")
        // }

        Plugboard { wiring }
    }
}
