extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

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
                    "/rate" => api.spawn(message.chat.text(format!("курс валют!"))),
                    _ => (),
                }
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
