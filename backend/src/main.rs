extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

use std::io;
use hyper::Client;
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use futures::future::Future;
use futures::prelude::*;
use serde_json::Value;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    //   let body = read_test_json().unwrap();

    //   parse_body(&*body);
    //
    from_server();
}

fn from_server() {
    let mut core = Core::new().unwrap();

    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let uri = "https://www.tinkoff.ru/api/v1/currency_rates/"
        .parse()
        .unwrap();

    let work = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            parse_body(&body);
            Ok(())
        })
    });

    core.run(work).unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct Rates {
    rates: Vec<Rate>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rate {
    category: String,
    buy: f32,
    #[serde(rename = "fromCurrency")]
    from_currency: Value,
}

fn parse_body(body: &[u8]) {
    let v: Value = serde_json::from_slice(&body)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .unwrap();
    let raw = &v["payload"];
    {
        let rates: Rates = serde_json::from_value(raw.clone()).unwrap();
        let buy = rates.rates.iter().find(|r| {
            r.category == "DebitCardsTransfers" && r.from_currency.get("code").unwrap() == 840
        });

        match buy {
            Some(rate) => println!("rate: {:?}", rate.buy),
            _ => println!("rate not found"),
        }
    }
}

#[allow(dead_code)]
fn read_test_json() -> Result<Box<Vec<u8>>, std::io::Error> {
    let mut f = File::open("test.json")?;
    let mut buffer = vec![0u8; 0];
    f.read_to_end(&mut buffer)?;

    let ret = buffer.clone();
    Ok(Box::new(ret))
}
