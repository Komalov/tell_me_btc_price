use cmc::Cmc;
use dotenv::dotenv;
use mac_notification_sys::*;

use std::env;

fn main() {
    dotenv().ok();

    let key = env::var("CMC_API_KEY").expect("No CMC key");

    let cmc = Cmc::new(key);

    match cmc.price("BTC") {
        Ok(price) => {
            let btc_price = format!("${:?}", price as u16);

            send_notification(
                "BTC price is",
                None,
                &btc_price,
                Some(Notification::new().sound("Blow")),
            )
            .unwrap();
        }
        Err(err) => println!("{}", err),
    }
}
