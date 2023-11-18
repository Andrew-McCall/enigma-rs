use crate::{
    enigma_int::{FromEnigmaInt, ToEnigmaInt},
    Enigma,
};

impl Enigma {
    pub fn encode(mut self, message: &String) -> String {
        let mut output = String::new();

        for c in message.chars() {
            if c == ' ' {
                output.push(' ');
                continue;
            }

            let mut internal_int = c.to_internal_int();

            let mut turn_next = true;
            for rotor in self.rotors.iter_mut().enumerate() {
                if turn_next {
                    rotor.1.turn();
                    turn_next = false;
                } else {
                    let position = rotor.1.position;
                    let turnover = rotor.1.turnover;

                    if position == turnover {
                        //turn_next = true;
                    }
                }

                internal_int = rotor.1.encode(internal_int);
            }

            internal_int = self.reflector.reflect(internal_int);

            self.rotors.reverse();
            for rotor in &self.rotors {
                internal_int = rotor.decode(internal_int);
            }

            output.push(internal_int.from_internal_int());
        }

        output
    }
}
