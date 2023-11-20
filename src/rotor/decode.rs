impl super::Rotor {
    pub fn decode(&self, input: usize) -> usize {
        (26 + self.wiring.iter().position(|&c| c == input).unwrap() - self.position) % 26
    }
}
