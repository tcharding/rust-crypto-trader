//! Connect to kraken API.

use coinnect::{
    error::Error,
    kraken::{KrakenApi, KrakenCreds},
};
use std::path::PathBuf;
// use rust_decimal::Decimal;
// use serde::{Deserialize, Serialize};
// use serde_json::value::Value;
// use std::{
//     path::{Path, PathBuf},
//     str::FromStr,
// };

#[derive(Debug)]
pub struct Api {
    api: KrakenApi,
}

impl Api {
    pub fn new(path: PathBuf) -> Result<Api, Error> {
        // We create a KrakenApi by loading a json file containing API configuration
        // (see documentation for more info)
        let my_creds =
            KrakenCreds::new_from_file("account_kraken", path).expect("failed to read creds");
        let api = KrakenApi::new(my_creds).expect("failed to create api");

        Ok(Api { api })
    }

    pub fn assert_public(&mut self) -> Result<(), Error> {
        let tp = "XXBTZUSD";
        let _ = self.api.get_order_book(tp, "1")?;

        Ok(())
    }
}

// pub fn foo() {
//     let tp = "XXBTZUSD";

//     let map = api
//         .get_order_book(tp, "100000")
//         .expect("failed to get order book");

//     let result = map.get("result").expect("no result");

//     let xbt = result.get(tp).expect("no XBT");
//     let asks = xbt.get("asks").expect("no asks");
//     let bids = xbt.get("bids").expect("no bids");

//     println!("ask[0]: {:?}", asks[0]);
//     println!("bid[0]: {:?}", bids[0]);

//     let mut a = vec![];
//     if let Value::Array(v) = asks {
//         for ask in v.iter() {
//             if let Value::Array(v) = ask {
//                 let mut price = serde_json::to_string(&v[0])?;
//                 price.pop();
//                 price = price[1..].to_string();

//                 let mut volume = serde_json::to_string(&v[1])?;
//                 volume.pop();
//                 volume = volume[1..].to_string();

//                 println!("{:?}", price);
//                 a.push(Ask {
//                     price: Decimal::from_str(&price).expect("price fail"),
//                     volume: Decimal::from_str(&volume).unwrap(),
//                     timestamp: v[2].as_u64().unwrap(),
//                 });
//             }
//         }
//     }

//     println!("{:?}", a);

//     Ok(())
// }

// #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
// struct Ask {
//     price: Decimal,
//     volume: Decimal,
//     timestamp: u64,
// }

// // fn split_ask(v: &Value) -> Result<(String, String)> {
// //     //    let a: Vec<Value> = serde_json::to_string(v)?;

// //     let price = v.get(0).expect("no price").to_string();
// //     let volume = v.get(1).expect("no volume").to_string();

// //     Ok((price, volume))
// // }
