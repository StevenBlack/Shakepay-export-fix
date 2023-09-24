![Shakepay logo](./shakepay.png)

# Shakepay-export-fix

[Shakepay](https://shakepay.com) allows Canadians to buy/sell bitcoin. And Canadians have to report their captial gains/losses to 
the CRA. This ought to be easy, but it's not.

Shakepay provides a CSV export of your transactions, but it's not in a format that can be easily imported into Excel or accounting software. 

This repository contains a utility, written in [Rust](https://www.rust-lang.org/), that converts the CSV file into a format that 
can be imported into Excel and other common finantial analysis and reporting software.

## Here's the problem

The .CSV file that Shakepay provides has an issues that make it difficult to import into accounting software.  The `Date` column is in a 
format that is not recognized by Excel.  The Shakepay date format appears to be [ISO 8601 in UTC](https://en.wikipedia.org/wiki/ISO_8601) but 
that's not documented by Shakepay.

Here's a sample chunk of CSV similar to what you'll get from Shakepay.  

```csv
"Transaction Type","Date","Amount Debited","Debit Currency","Amount Credited","Credit Currency","Buy / Sell Rate","Direction","Spot Rate","Source / Destination","Blockchain Transaction ID"
"fiat funding","2022-09-10T00:36:12+00",,,500,"CAD",,"credit",,"myemail@example.com",
"purchase/sale","2022-09-10T00:56:41+00",250,"CAD",0.00405049,"BTC","61720.8993","purchase",,,
"crypto cashout","2022-09-10T00:58:55+00",0.00405049,"BTC",,,,"debit","61027.7612","bc1q6f5b95fe8cc165adad7bb399dd7416f25f08348dc0f7cdbdbca6b01ca9","887534e0dbe0af1c77ea5b7e45876dd40b5e9664f1bce7384071023406e2729d"
"other","2022-09-10T15:31:24+00",,,30,"CAD",,"credit",,,
"purchase/sale","2022-09-10T18:40:33+00",280,"CAD",0.00453538,"BTC","61736.7866","purchase",,,
"crypto cashout","2022-09-10T18:40:59+00",0.00453538,"BTC",,,,"debit","61056.3802","bc1q6f5b95fe8cc165adad7bb399dd7416f25f08348dc0f7cdbdbca6b01ca9","3ac40584dce257179e057c67e6269065bc174a9a5392b4ed609d041d9c594266"
```


The resulting `Date` string, for example `2022-09-10T00:36:12+00`, is ultra convenient for shakepay because this is commonly used in vanilla
database dumps.  However, it's not a format that is recognized by Excel.  Excel expects a date in the format `2022-09-10 00:36:12` and,
for the most part, the substring `2022-09-10` is ample for consumer-level reporting and consolidating.

So if canadians want to make use of the Shakepay CSV export, they have to manually edit the file until Excel recognizes the date format, or
fuss with Excel import config settings.  All this should be easy, but it's not.

This repository fixes that problem.
