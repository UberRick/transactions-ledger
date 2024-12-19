use std::collections::HashMap;

use super::models::{Account, Transaction, TransactionKind};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct TxKey {
    acc_id: u16,
    tx_id: u32,
}

pub struct Ledger {
    pub accounts: HashMap<u16, Account>,
    pub deposits: HashMap<TxKey, Transaction>,
    pub disputes: HashMap<TxKey, u16>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            accounts: HashMap::new(),
            deposits: HashMap::new(),
            disputes: HashMap::new(),
        }
    }

    pub fn process_transaction(&mut self, tx: Transaction) {
        let account = self
            .accounts
            .entry(tx.acc_id)
            .or_insert_with(|| Account::new(tx.acc_id));

        if account.locked {
            return;
        }

        let tx_key = TxKey {
            acc_id: tx.acc_id,
            tx_id: tx.tx_id,
        };

        match tx.kind {
            TransactionKind::Deposit { amount } => {
                account.available += amount;
                account.total += amount;
                self.deposits.insert(tx_key, tx.clone());
            }
            TransactionKind::Withdrawal { amount } => {
                if account.available < amount {
                    return;
                }
                account.available -= amount;
                account.total -= amount;
            }
            TransactionKind::Dispute => {
                let ref_tx = self.deposits.get(&tx_key).map(|tx| tx.kind.clone());

                if let Some(TransactionKind::Deposit { amount }) = ref_tx {
                    account.available -= amount;
                    account.held += amount;
                    self.disputes.insert(tx_key, tx.acc_id);
                }
            }
            TransactionKind::Resolve => {
                if self.disputes.remove(&tx_key).is_some() {
                    let ref_tx = self.deposits.get(&tx_key).map(|tx| tx.kind.clone());

                    if let Some(TransactionKind::Deposit { amount }) = ref_tx {
                        account.available += amount;
                        account.held -= amount;
                    }
                }
            }
            TransactionKind::Chargeback => {
                if self.disputes.contains_key(&tx_key) {
                    let ref_tx = self.deposits.get(&tx_key).map(|tx| tx.kind.clone());

                    if let Some(TransactionKind::Deposit { amount }) = ref_tx {
                        account.held -= amount;
                        account.total -= amount;
                        account.locked = true;
                    }
                }
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

        for tx in transactions {
            ledger.process_transaction(tx);
        }

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

        for tx in transactions {
            ledger.process_transaction(tx);
        }

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

        for tx in transactions {
            ledger.process_transaction(tx);
        }

        assert_eq!(ledger.deposits.len(), 2);
    }

    #[test]
    fn test_process_transactions_with_dispute_without_matching_client() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 2,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Dispute,
                acc_id: 2,
            },
        ];

        for tx in transactions {
            ledger.process_transaction(tx);
        }

        let account = ledger.accounts.get(&1).unwrap();
        let incorrect_account = ledger.accounts.get(&2).unwrap();

        assert_eq!(account.available, dec!(4.0));
        assert_eq!(account.held, dec!(0.0));
        assert_eq!(account.total, dec!(4.0));
        assert_eq!(incorrect_account.available, dec!(0.0));
        assert_eq!(incorrect_account.held, dec!(0.0));
        assert_eq!(incorrect_account.total, dec!(0.0));
    }

    #[test]
    fn test_process_transactions_with_dispute() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 2,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Dispute,
                acc_id: 1,
            },
        ];

        for tx in transactions {
            ledger.process_transaction(tx);
        }

        let account = ledger.accounts.get(&1).unwrap();

        assert_eq!(account.available, dec!(2.0));
        assert_eq!(account.held, dec!(2.0));
        assert_eq!(account.total, dec!(4.0));
    }

    #[test]
    fn test_process_transactions_with_resolve() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Dispute,
                acc_id: 1,
            },
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Resolve,
                acc_id: 1,
            },
        ];

        for tx in transactions {
            ledger.process_transaction(tx);
        }

        let account = ledger.accounts.get(&1).unwrap();
        let tx_key = TxKey {
            acc_id: 1,
            tx_id: 1,
        };
        assert_eq!(account.available, dec!(2.0));
        assert_eq!(account.held, dec!(0.0));
        assert_eq!(account.total, dec!(2.0));
        assert!(!ledger.disputes.contains_key(&tx_key));
    }

    #[test]
    fn test_process_transactions_with_chargeback() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Dispute,
                acc_id: 1,
            },
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Chargeback,
                acc_id: 1,
            },
        ];

        for tx in transactions {
            ledger.process_transaction(tx);
        }

        let account = ledger.accounts.get(&1).unwrap();
        let tx_key = TxKey {
            acc_id: 1,
            tx_id: 1,
        };
        assert_eq!(account.available, dec!(0.0));
        assert_eq!(account.held, dec!(0.0));
        assert_eq!(account.total, dec!(0.0));
        assert_eq!(account.locked, true);
        assert!(ledger.disputes.contains_key(&tx_key));
    }

    #[test]
    fn test_processing_transactions_with_locked_account() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Dispute,
                acc_id: 1,
            },
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Chargeback,
                acc_id: 1,
            },
            Transaction {
                tx_id: 2,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
        ];

        for tx in transactions {
            ledger.process_transaction(tx);
        }

        let account = ledger.accounts.get(&1).unwrap();
        assert_eq!(account.available, dec!(0.0));
        assert_eq!(account.held, dec!(0.0));
        assert_eq!(account.total, dec!(0.0));
        assert_eq!(account.locked, true);
    }

    #[test]
    fn test_processing_transactions_with_insufficient_funds() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 2,
                kind: TransactionKind::Withdrawal { amount: dec!(3.0) },
                acc_id: 1,
            },
        ];

        for tx in transactions {
            ledger.process_transaction(tx);
        }

        let account = ledger.accounts.get(&1).unwrap();
        assert_eq!(account.available, dec!(2.0));
        assert_eq!(account.held, dec!(0.0));
        assert_eq!(account.total, dec!(2.0));
    }

    #[test]
    fn test_processing_disput_with_non_existant_deposit() {
        let mut ledger = Ledger::new();
        let transactions = vec![Transaction {
            tx_id: 1,
            kind: TransactionKind::Dispute,
            acc_id: 1,
        }];

        for tx in transactions {
            ledger.process_transaction(tx);
        }

        let account = ledger.accounts.get(&1).unwrap();
        assert_eq!(account.available, dec!(0.0));
        assert_eq!(account.held, dec!(0.0));
        assert_eq!(account.total, dec!(0.0));
    }

    #[test]
    fn test_processing_resolve_with_non_existent_dispute() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Resolve,
                acc_id: 1,
            },
        ];

        for tx in transactions {
            ledger.process_transaction(tx);
        }

        let account = ledger.accounts.get(&1).unwrap();
        assert_eq!(account.available, dec!(2.0));
        assert_eq!(account.held, dec!(0.0));
        assert_eq!(account.total, dec!(2.0));
    }

    #[test]
    fn test_processing_chargeback_with_non_existent_dispute() {
        let mut ledger = Ledger::new();
        let transactions = vec![
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Deposit { amount: dec!(2.0) },
                acc_id: 1,
            },
            Transaction {
                tx_id: 1,
                kind: TransactionKind::Chargeback,
                acc_id: 1,
            },
        ];

        for tx in transactions {
            ledger.process_transaction(tx);
        }

        let account = ledger.accounts.get(&1).unwrap();
        assert_eq!(account.available, dec!(2.0));
        assert_eq!(account.held, dec!(0.0));
        assert_eq!(account.total, dec!(2.0));
    }
}
