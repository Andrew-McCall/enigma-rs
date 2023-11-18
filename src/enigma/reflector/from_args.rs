use crate::enigma_int::ToEnigmaInt;

use super::Reflector;

impl Reflector {
    pub fn from_args(argument: &String) -> Reflector {
        let pairs = argument.split_whitespace();
        if pairs.clone().count() != 13 {
            panic!("Reflector must have 13 pairs of letters.\n --reflector \"PAIRS\"\n --reflector \"AB CD EF GH IJ KL MN OP QR ST UV WX YZ\"");
        }

        let mut wiring = [0; 26];

        for pair in pairs {
            let mut chars = pair.chars();
            let first = chars.next().unwrap();
            let second = chars.next().unwrap();

            if !first.is_ascii_alphabetic() || !second.is_ascii_alphabetic() {
                panic!("{} is invalid. Reflector pairs must be letters.", pair);
            }

            let first_int = first.to_internal_int();
            let second_int = second.to_internal_int();

            if wiring[first_int] != 0 || wiring[second_int] != 0 || first_int == second_int {
                panic!("Reflector pairs must not have duplicate letters.");
            }

            wiring[first_int] = second_int;
            wiring[second_int] = first_int;
        }

        Reflector { wiring }
    }
}
