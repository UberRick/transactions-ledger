use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use clap::Parser;
use csv::Trim;

use self::transactions::ledger::Ledger;

mod transactions;

#[derive(Parser)]
struct Args {
    #[clap()]
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file = File::open(args.file)?;

    let mut ledger = Ledger::new();
    process_transactions(file, &mut ledger)?;

    write_ledger_accounts(&ledger, std::io::stdout())
}

fn process_transactions<R: Read>(reader: R, ledger: &mut Ledger) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(reader);
    for result in rdr.records() {
        let record = result?;
        let transaction = transactions::parser::parse(&record)?;
        ledger.process_transaction(transaction);
    }
    Ok(())
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

        // Process transactions
        let mut ledger = Ledger::new();
        process_transactions(input_csv.as_bytes(), &mut ledger)
            .expect("error processing transactions");

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
