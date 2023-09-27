extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;
use std::fmt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Transaction Type")]
    transaction_type: String,

    #[serde(rename = "Date")]
    date: String,

    #[serde(rename = "Amount Debited")]
    #[serde(deserialize_with = "csv::invalid_option")]
    amount_debited: Option<f64>,

    #[serde(rename = "Debit Currency")]
    debit_currency: String,

    #[serde(rename = "Amount Credited")]
    #[serde(deserialize_with = "csv::invalid_option")]
    amount_credited: Option<f64>,

    #[allow(dead_code)]
    #[serde(rename = "Credit Currency")]
    credit_currency: String,

    #[serde(rename = "Buy / Sell Rate")]
    #[serde(deserialize_with = "csv::invalid_option")]
    buy_sell_rate: Option<f64>,

    #[allow(dead_code)]
    #[serde(rename = "Direction")]
    direction: String,

    #[allow(dead_code)]
    #[serde(rename = "Spot Rate")]
    spot_rate: Option<f64>,

    #[allow(dead_code)]
    #[serde(rename = "Source / Destination")]
    source_destination: String,

    #[allow(dead_code)]
    #[serde(rename = "Blockchain Transaction ID")]
    blockchain_tx_id: String,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let trans = {
            match self.transaction_type.as_str() {
                "fiat funding" => "Fund account".to_string(),
                "crypto cashout" => "Move out".to_string(),
                "purchase/sale" => "Buy".to_string(),
                "other" => "Other".to_string(),
                _ => "Unknown tx type".to_string(),
            }
        };
        write!(f,
            "\"{}\",\"{}\",{},\"{}\",{},\"{}\",{},\"{}\",{},\"{}\",\"{}\"",
            date_fix(&self.date),
            trans,
            self.amount_debited.unwrap_or(0.),
            self.debit_currency,
            self.amount_credited.unwrap_or(0.),
            self.credit_currency,
            self.buy_sell_rate.unwrap_or(0.),
            self.direction,
            self.spot_rate.unwrap_or(0.),
            self.source_destination,
            self.blockchain_tx_id
        )?;
        Ok(())
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .double_quote(false)
        // .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_path(file_path)
        .map_err(|err| format!("Failed to read CSV file: {}", err))?;

    // print out headers
    println!("\"date\", \"transaction_type\", \"amount_debited\", \"debit_currency\", \"amount_credited\", \"credit_currency\", \"buy_sell_rate\", \"direction\", \"spot_rate\", \"source_destination\", \"blockchain_tx_id\"");

    // print each row of data
    {
        for result in rdr.deserialize() {
            let record: Record = result?;
            println!("{}", record);
        }
    }
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
