mod decode;
mod encode;
mod from_args;
mod get_position;
mod turn;

pub struct Rotor {
    pub wiring: Vec<usize>,
    pub turnover: usize,
    pub position: usize,
}
