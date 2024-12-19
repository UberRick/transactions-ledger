use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use self::transactions::ledger::Ledger;
use self::transactions::models::Transaction;

mod transactions;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path_from_args()?;
    let file = File::open(file_path)?;

    let transactions = read_transactions(file)?;

    let mut ledger = Ledger::new();
    ledger.process_transactions(transactions);

    write_ledger_accounts(&ledger, std::io::stdout())
}

fn get_file_path_from_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage: cargo run -- transactions.csv".into());
    }

    args.get(1).cloned().ok_or("error parsing file path".into())
}

fn read_transactions<R: Read>(reader: R) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.records()
        .map(|record| {
            let record = record?;
            transactions::parser::parse(&record)
        })
        .collect()
}

fn write_ledger_accounts<W: Write>(ledger: &Ledger, writer: W) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(writer);
    for account in ledger.accounts.values() {
        wtr.serialize(account)?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_ledger_accounts() {
        let input_csv = r#"type,client,tx,amount
deposit,1,1,1.0
deposit,2,2,2.0
deposit,1,3,2.0
withdrawal,1,4,1.5
withdrawal,2,5,3.0
deposit,3,6,1.0
deposit,3,7,2.0
dispute,3,7,
deposit,4,8,2.3456
deposit,4,9,0.1111
dispute,4,8,
resolve,4,8,
deposit,5,10,2.0
dispute,5,10,
chargeback,5,10,
"#;

        // Parse transactions
        let transactions =
            read_transactions(input_csv.as_bytes()).expect("error reading transactions");

        // Process transactions
        let mut ledger = Ledger::new();
        ledger.process_transactions(transactions);

        // Mock writer
        let mut buffer = Vec::new();
        write_ledger_accounts(&ledger, &mut buffer).expect("error writing to buffer");

        let actual_output = String::from_utf8(buffer).expect("error converting buffer to string");

        // As we are using a hashmap, the order isn't guaranteed
        assert!(
            actual_output.contains("1,1.5,0.0,1.5,false"),
            "{}",
            actual_output
        );
        assert!(
            actual_output.contains("2,2.0,0.0,2.0,false"),
            "{}",
            actual_output
        );
        assert!(
            actual_output.contains("3,1.0,2.0,3.0,false"),
            "{}",
            actual_output
        );
        assert!(
            actual_output.contains("4,2.4567,0.0000,2.4567,false"),
            "{}",
            actual_output
        );
        assert!(
            actual_output.contains("5,0.0,0.0,0.0,true"),
            "{}",
            actual_output
        );
    }
}
