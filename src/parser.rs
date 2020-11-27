use crate::model::*;
use chrono::*;
use csv::{ReaderBuilder, StringRecord};
use currency::Currency;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::File;

pub fn parse_file() -> csv::Result<Vec<Booking>> {
    let file = File::open("exampledata/Buchungsliste.csv")?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(transcoded);
    let mut parsed_bookings = Vec::new();
    for result in rdr.records() {
        let entry = parse_entry(result?);
        if entry.buchungsdatum >= NaiveDate::from_ymd(2020, 08, 01) {
            parsed_bookings.push(entry);
        }
    }
    Ok(parsed_bookings)
}

fn parse_entry(record: StringRecord) -> Booking {
    Booking {
        buchungsdatum: parse_naive_date(&record[0]),
        empfaenger: String::from(&record[1]),
        verwendungszweck: String::from(&record[2]),
        buchungstext: String::from(&record[3]),
        betrag: parse_currency(&record[4]),
        iban: String::from(&record[5]),
        bic: String::from(&record[6]),
        kategorie: String::from(&record[7]),
        notiz: String::from(&record[9]),
        schlagworte: String::from(&record[9]),
    }
}

pub fn parse_naive_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(&date, "%d.%m.%Y").expect("parse error")
}

pub fn parse_currency(currency: &str) -> Currency {
    Currency::from_str(&["€", currency].join("")).expect("parse error")
}
