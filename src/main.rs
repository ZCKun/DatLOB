use std::{collections::BTreeMap, io::BufRead};
use book::Book;
use types::Side;

use crate::types::{Order, Trade, TradeType};
use std::io::{BufReader, Read};
use byteorder::{LittleEndian, ByteOrder};
use crate::dat_reader::{Header, DatReader};
use crate::mdt_type::DataType;

mod book;
mod types;
mod dat_reader;
mod mdt_type;
mod mdt_struct;

fn test_from_csv(book: &mut Book) {
    if let Ok(lines) = read_lines("files/order.csv") {
        for line in lines {
            if let Ok(row) = line {
                let items: Vec<&str> = row.split(',').collect();
                let time = items[1].parse::<i64>().unwrap();
                let id = items[2].parse::<i64>().unwrap();
                let side = if items[3] == "BUY" {Side::BID} else {Side::ASK};
                let price = items[4].parse::<f64>().unwrap();
                let qty = items[5].parse::<i64>().unwrap();

                let order = Order{ time, id, side, price, qty };

                book.add_order(&order);
            }
        }
    }

    if let Ok(lines) = read_lines("files/trade.csv") {
        for line in lines {
            if let Ok(row) = line {
                let items: Vec<&str> = row.split(',').collect();
                let time = items[1].parse::<i64>().unwrap();
                let id = items[2].parse::<i64>().unwrap();
                let bid_id = items[3].parse::<i64>().unwrap();
                let ask_id = items[4].parse::<i64>().unwrap();
                let price = items[6].parse::<f64>().unwrap();
                let qty = items[7].parse::<i64>().unwrap();

                let trade_type = if ask_id != 0 && bid_id != 0 { TradeType::TRADED } else { TradeType::CACNEL };

                let trade = Trade{ time, id, price, qty, ask_id, bid_id, trade_type };

                book.on_trade(&trade);

                println!("{}", book)
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Item {
    data_type: i32,
    time: i64,
    id: i64,
    side: Option<String>,
    price: f64,
    qty: i64,
    bid_id: Option<i64>,
    ask_id: Option<i64>
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Data {
    local_time:String, 
    item:Item 
}

fn test_from_json(book: &mut Book) {
    let content = std::fs::read_to_string("files/data.json").expect("open file failed.");
    let data: BTreeMap<&str, Item> = serde_json::from_str(content.as_str()).expect("parser data failed.");
    for (_, item) in data {
        if item.price == 0.0 {
            continue;
        }

        if item.data_type == 0 {
            let order = Order {
                time: item.time,
                id: item.id,
                price: item.price,
                qty: item.qty,
                side: if item.side.unwrap() == "BUY" { Side::BID } else {Side::ASK }
            };
            book.add_order(&order);
        } else {
            let trade_type = if item.ask_id.unwrap() == 0 || item.bid_id.unwrap() == 0 { TradeType::CACNEL } else {TradeType::TRADED };
            let trade = Trade{
                time: item.time,
                id: item.id,
                price: item.price,
                qty: item.qty,
                ask_id: item.ask_id.unwrap(),
                bid_id: item.bid_id.unwrap(),
                trade_type
            };
            book.on_trade(&trade);
            println!("{}\n{}", book, item.time)
        }
    }
}

fn main() {
    // let mut book = Book::new();
    // test_from_csv(&mut book);
    // test_from_json(&mut book);
    // let file_path = "C:/Users/x2h1z/Desktop/Data/dat/aa/202108020705.dat";
    let file_path = "/Users/2h0x/Data/dat/202107140705.dat";
    let mut reader = DatReader::new(file_path);
    reader.read();

    println!("process done!")
}

fn read_lines<P>(filepath: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where P: AsRef<std::path::Path>, {
    let file = std::fs::File::open(filepath).expect("open file failed.");
    Ok(std::io::BufReader::new(file).lines())
}
