use std::error::Error;

use rust_decimal::Decimal;

use super::models::{Transaction, TransactionKind};

pub fn parse(record: &csv::StringRecord) -> Result<Transaction, Box<dyn Error>> {
    let tx_kind = record.get(0).ok_or("error parsing type")?.trim();

    let account_id = record
        .get(1)
        .ok_or("error parsing account_id")?
        .parse::<u16>()?;
    let id = record
        .get(2)
        .ok_or("error parsing transaction id")?
        .parse::<u32>()?;

    let amount = record.get(3).and_then(|s| s.parse::<Decimal>().ok());

    let kind = match tx_kind {
        "deposit" => {
            let amount = amount.ok_or("error parsing amount for deposit")?;
            TransactionKind::Deposit { amount }
        }
        "withdrawal" => {
            let amount = amount.ok_or("error parsing amount for withdrawal")?;
            TransactionKind::Withdrawal { amount }
        }
        "dispute" => TransactionKind::Dispute,
        "resolve" => TransactionKind::Resolve,
        "chargeback" => TransactionKind::Chargeback,
        _ => {
            return Err("Invalid transaction type".into());
        }
    };

    Ok(Transaction {
        tx_id: id,
        kind,
        acc_id: account_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    #[test]
    fn test_parse() {
        let d = csv::StringRecord::from(vec!["deposit", "1", "1", "1.0"]);
        let dt = super::parse(&d).unwrap();
        assert_eq!(dt.kind, TransactionKind::Deposit { amount: dec!(1.0) });
        assert_eq!(dt.acc_id, 1);
        assert_eq!(dt.tx_id, 1);

        let w = csv::StringRecord::from(vec!["withdrawal", "2", "2", "2.0"]);
        let wt = super::parse(&w).unwrap();
        assert_eq!(wt.kind, TransactionKind::Withdrawal { amount: dec!(2.0) });
        assert_eq!(wt.acc_id, 2);
        assert_eq!(wt.tx_id, 2);

        let d = csv::StringRecord::from(vec!["dispute", "3", "1"]);
        let dt = super::parse(&d).unwrap();
        assert_eq!(dt.kind, TransactionKind::Dispute);
        assert_eq!(dt.acc_id, 3);
        assert_eq!(dt.tx_id, 1);

        let r = csv::StringRecord::from(vec!["resolve", "4", "2"]);
        let rt = super::parse(&r).unwrap();
        assert_eq!(rt.kind, TransactionKind::Resolve);
        assert_eq!(rt.acc_id, 4);
        assert_eq!(rt.tx_id, 2);

        let c = csv::StringRecord::from(vec!["chargeback", "5", "1"]);
        let ct = super::parse(&c).unwrap();
        assert_eq!(ct.kind, TransactionKind::Chargeback);
        assert_eq!(ct.acc_id, 5);
        assert_eq!(ct.tx_id, 1);
    }

    #[test]
    fn test_parse_amounts_with_4dp() {
        let d = csv::StringRecord::from(vec!["deposit", "1", "1", "1.1234"]);
        let dt = super::parse(&d).unwrap();
        assert_eq!(
            dt.kind,
            TransactionKind::Deposit {
                amount: dec!(1.1234)
            }
        );
        assert_eq!(dt.acc_id, 1);
        assert_eq!(dt.tx_id, 1);
    }

    #[test]
    fn test_parse_no_amount_for_deposit() {
        let record = csv::StringRecord::from(vec!["deposit", "1", "1"]);
        let transaction = super::parse(&record);
        assert!(transaction.is_err());
    }

    #[test]
    fn test_parse_no_amount_for_withdrawal() {
        let record = csv::StringRecord::from(vec!["withdrawal", "1", "1"]);
        let transaction = super::parse(&record);
        assert!(transaction.is_err());
    }

    #[test]
    fn test_parse_invalid_type() {
        let record = csv::StringRecord::from(vec!["invalid", "1", "1", "1.0"]);
        let transaction = super::parse(&record);
        assert!(transaction.is_err());
    }

    #[test]
    fn test_parse_invalid_account_id() {
        let record = csv::StringRecord::from(vec!["deposit", "invalid", "1", "1.0"]);
        let transaction = super::parse(&record);
        assert!(transaction.is_err());
    }

    #[test]
    fn test_parse_invalid_id() {
        let record = csv::StringRecord::from(vec!["deposit", "1", "invalid", "1.0"]);
        let transaction = super::parse(&record);
        assert!(transaction.is_err());
    }
}
