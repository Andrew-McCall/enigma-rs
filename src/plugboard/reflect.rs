impl super::Plugboard {
    pub fn reflect(&self, input: usize) -> usize {
        self.wiring[input]
    }
}
