use crate::transaction::{Transaction, TxType};

#[derive(Debug)]
pub struct Account {
    pub client: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
    transactions: Vec<Transaction>,
}

impl Account {
    pub fn new(id: u16) -> Self {
        Self {
            client: id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
            transactions: Vec::new(),
        }
    }

    fn update_available(&mut self) {
        self.available = ((self.total - self.held) * 10000.0).round() / 10000.0;
    }

    fn deposit(&mut self, new_tx: Transaction) {
        self.total += new_tx.get_rounded_amount();
    }

    fn withdraw(&mut self, new_tx: Transaction) {
        if self.available >= new_tx.get_rounded_amount() {
            self.total -= new_tx.get_rounded_amount();
        }
    }

    fn dispute(&mut self, new_tx: Transaction) {
        if let Some(original_tx) = self.transactions.iter().find(|t| t.tx == new_tx.tx) {
            self.held += original_tx.get_rounded_amount();
        }
    }

    fn resolve(&mut self, new_tx: Transaction) {
        if self.transaction_disputed(new_tx.tx) {
            if let Some(original_tx) = self.transactions.iter().find(|t| t.tx == new_tx.tx) {
                self.held -= original_tx.get_rounded_amount();
            }
        }
    }

    fn chargeback(&mut self, new_tx: Transaction) {
        if self.transaction_disputed(new_tx.tx) {
            if let Some(original_tx) = self.transactions.iter().find(|t| {
                t.tx == new_tx.tx
                    && (t.tx_type.eq(&TxType::Deposit) || t.tx_type.eq(&TxType::Withdrawal))
            }) {
                self.locked = true;
                self.held -= original_tx.get_rounded_amount();

                match original_tx.tx_type {
                    TxType::Deposit => self.total -= original_tx.get_rounded_amount(),
                    TxType::Withdrawal => self.total += original_tx.get_rounded_amount(),
                    _ => (),
                }
            }
        }
    }

    fn transaction_disputed(&self, tx_id: u32) -> bool {
        self.transactions
            .iter()
            .any(|t| t.tx == tx_id && t.tx_type.eq(&TxType::Dispute))
    }

    pub fn add_transaction(&mut self, new_tx: Transaction) {
        match new_tx.tx_type {
            TxType::Deposit => self.deposit(new_tx),
            TxType::Withdrawal => self.withdraw(new_tx),
            TxType::Dispute => self.dispute(new_tx),
            TxType::Resolve => self.resolve(new_tx),
            TxType::Chargeback => self.chargeback(new_tx),
        }
        self.transactions.push(new_tx);
        self.update_available();
    }
}
