use crate::Rotor;

impl Rotor {
    pub fn encode(&self, input: usize) -> usize {
        self.wiring[(input + self.position) % 26]
    }

    pub fn turn(&mut self) {
        self.position = (self.position + 1) % 26;
    }
}
