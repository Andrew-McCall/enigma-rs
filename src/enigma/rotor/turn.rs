impl super::Rotor {
    pub fn turn(&mut self) {
        self.position = (self.position + 1) % 26;
    }
}
