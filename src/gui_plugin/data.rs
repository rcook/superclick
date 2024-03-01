use std::sync::{Arc, Mutex};

pub type TransportInfoRef = Arc<Mutex<TransportInfo>>;

#[derive(Default)]
pub struct TransportInfo {
    pub bar_number: i32,
    pub bar_start_pos_beats: f64,
    pub pos_beats: f64,
    pub time_sig_numerator: i32,
    pub time_sig_denominator: i32,
}

impl TransportInfo {
    pub fn new() -> TransportInfoRef {
        Arc::new(Mutex::new(Self::default()))
    }
}
