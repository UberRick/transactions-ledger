use std::collections::HashMap;

use super::models::{Account, Transaction, TransactionKind};

pub struct Ledger {
    pub accounts: HashMap<u16, Account>,
    pub deposits: HashMap<u32, Transaction>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            accounts: HashMap::new(),
            deposits: HashMap::new(),
        }
    }

    pub fn process_transactions(&mut self, txs: Vec<Transaction>) {
        for tx in txs {
            let account = self
                .accounts
                .entry(tx.acc_id)
                .or_insert_with(|| Account::new(tx.acc_id));

            match tx.kind {
                TransactionKind::Deposit { amount } => {
                    account.available += amount;
                    account.total += amount;
                    self.deposits.insert(tx.tx_id, tx.clone());
                }
                TransactionKind::Withdrawal { amount } => {
                    if account.available < amount {
                        continue;
                    }
                    account.available -= amount;
                    account.total -= amount;
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    #[test]
    fn test_process_transactions() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 2,
                kind: TransactionKind::Withdrawal { amount: dec!(1.0) },
                acc_id: 1,
            },
        ];

        ledger.process_transactions(transactions);

        let account = ledger.accounts.get(&1).unwrap();
        assert_eq!(account.available, dec!(1.0));
        assert_eq!(account.held, dec!(0.0));
        assert_eq!(account.total, dec!(1.0));
    }

    #[test]
    fn test_processing_transactions_with_4dp() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.1) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 2,
                kind: TransactionKind::Withdrawal { amount: dec!(1.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 3,
                kind: TransactionKind::Deposit {
                    amount: dec!(1.2345),
                },
                acc_id: 1,
            },
        ];

        ledger.process_transactions(transactions);

        let account = ledger.accounts.get(&1).unwrap();
        assert_eq!(account.available, dec!(2.3345));
        assert_eq!(account.held, dec!(0.0));
        assert_eq!(account.total, dec!(2.3345));
    }

    #[test]
    fn test_deposits_are_stored() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 2,
                kind: TransactionKind::Deposit { amount: dec!(1.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 3,
                kind: TransactionKind::Withdrawal { amount: dec!(1.0) },
                acc_id: 1,
            },
        ];

        ledger.process_transactions(transactions);

        assert_eq!(ledger.deposits.len(), 2);
    }
}
