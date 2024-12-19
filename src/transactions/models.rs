pub enum TransactionKind {
    Deposit { amount: f64 },
    Withdrawal { amount: f64 },
    Dispute,
    Resolve,
    Chargeback,
}

pub struct Transaction {
    pub tx_id: u32,
    pub kind: TransactionKind,
    pub acc_id: u16,
}

pub struct Account {
    pub id: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

impl Account {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}
