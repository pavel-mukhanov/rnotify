extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("RNOTIFY_BOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {

        println!("update {:?}", update);

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text { ref data, .. } = message.kind {
                println!("<{}>: {}", &message.from.first_name, data);

                match data.as_ref() {
                    "/rate" => {
                        let rate = rate_from_bd();
                        api.spawn(message.chat.text(format!(
                            "курс валют 1$ = {:?}₽",
                            rate.unwrap()
                        )));
                    }
                    _ => (),
                }
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}

fn rate_from_bd() -> Result<f32, std::io::Error> {
    let mut file = File::open("rate.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    contents.parse::<f32>().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, "oh no!")
    })
}
