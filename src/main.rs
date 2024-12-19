use std::env;
use std::error::Error;
use std::fs::File;

use self::transactions::ledger::Ledger;
use self::transactions::models::Transaction;

mod transactions;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path_from_args()?;
    let file = File::open(file_path)?;

    let transactions = read_transactions(file)?;

    let mut ledger = Ledger::new();
    ledger.process_transactions(transactions);

    write_ledger_accounts(&ledger)
}

fn get_file_path_from_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage: cargo run -- transactions.csv".into());
    }

    args.get(1).cloned().ok_or("error parsing file path".into())
}

fn read_transactions(file: File) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(file);
    rdr.records()
        .map(|record| {
            let record = record?;
            transactions::parser::parse(&record)
        })
        .collect()
}

fn write_ledger_accounts(ledger: &Ledger) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    for account in ledger.accounts.values() {
        wtr.serialize(account)?;
    }
    wtr.flush()?;
    Ok(())
}
