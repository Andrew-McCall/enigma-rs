impl super::Rotor {
    pub fn encode(&self, input: usize) -> usize {
        self.wiring[(input + self.position) % 26]
    }
}
