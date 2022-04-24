use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub alias: String,
    pub short_id: String,
    pub spendable_msatoshis: u64,
    pub receivable_msatoshis: u64,
}

impl Channel {
    pub fn calculate_ratio(&self) -> f64 {
        self.spendable_msatoshis as f64 / self.receivable_msatoshis as f64
    }
}
