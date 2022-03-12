extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;
use std::fmt;
use serde::Deserialize;
use std::io;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Transaction Type")]
    transaction_type: String,
    #[serde(rename = "date")]
    date: String,
    #[serde(rename = "Amount Debited")]
    amount_debited: f64,
    #[serde(rename = "Debit Currency")]
    debit_currency: String,
    #[serde(rename = "Amount_Credited")]
    amount_credited: f64,
    #[allow(dead_code)]
    #[serde(rename = "Credit Currency")]
    credit_currency: String,
    #[serde(rename = "Buy / Sell Rate")]
    buy_sell_rate: f64,
    #[allow(dead_code)]
    #[serde(rename = "Direction")]
    direction: String,
    #[allow(dead_code)]
    #[serde(rename = "Spot Rate")]
    spot_rate: f64,
    #[allow(dead_code)]
    #[serde(rename = "Source / Destination")]
    source_destination: String,
    #[allow(dead_code)]
    #[serde(rename = "Blockchain Transaction ID")]
    blockchain_tx_id: String,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let trans = if self.transaction_type == "purchase/sale" && self.debit_currency == "CAD" {
            "Buy"
        } else {
            "Sell"
        };
        let coins = if trans == "Buy" {
            self.amount_credited
        } else {
            self.amount_debited
        };
        let can_dollars_per_coin = self.buy_sell_rate;
        let fee = 2.5f64;
        write!(f,
            "{},{},{}, ,{}, , ,{}, , , , , ,Shakepay",
            date_fix(&self.date),
            trans,
            coins,
            can_dollars_per_coin,
            fee,
        )
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .double_quote(false)
        // .has_headers(true)
        .from_path(file_path)?;

    let mut wtr = csv::Writer::from_writer(io::stdout());

    // the header line
    if false {
        // We nest this call in its own scope because of lifetimes.
        let headers = rdr.headers()?;
        wtr.write_record(headers)?;
        wtr.flush()?;
        // println!("{:?}", headers);
    }

    // the data
    for result in rdr.deserialize() {
        let record: Record = result?;
        if record.transaction_type == "purchase/sale" {
            // wtr.write_record(record)?;
            println!("{}", record);
        }
    }
    // wtr.flush()?;
    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn date_fix(d: &str ) -> String {
    // d looks like this:
    // "2021-12-10T18:40:59+00"
    format!("{}", d[0..10].to_string())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_fix() {
        assert_eq!(date_fix("2021-12-10T18:40:59+00"), "2021-12-10".to_string() );
    }
}
