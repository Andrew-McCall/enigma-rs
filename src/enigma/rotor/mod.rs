mod decode;
mod encode;
mod turn;

pub struct Rotor {
    pub wiring: Vec<usize>,
    pub turnover: usize,
    pub position: usize,
}
