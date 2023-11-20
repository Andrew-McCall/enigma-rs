impl super::Reflector {
    pub fn reflect(&self, input: usize) -> usize {
        self.wiring[input]
    }
}
