use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TxType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>,
}

impl Transaction {
    pub fn get_rounded_amount(&self) -> f32 {
        (self.amount.unwrap_or(0.0) * 10000.0).round() / 10000.0
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
