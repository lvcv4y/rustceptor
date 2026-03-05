#[macro_use] extern crate rocket;

use std::sync::LazyLock;
use rocket::tokio::sync::broadcast::{channel, Sender};
use random_string::generate;

pub mod routes;
pub mod models;
pub mod dyn_content;
pub mod capture;

use crate::routes::{routes, catchers};

// TODO use env + secret.
pub static MASTER_KEY: LazyLock<String> = LazyLock::new(|| {
    generate(40, "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890")
});

pub struct EventChannels {
    // Only one for now
    captured_reqs: Sender<String>, // Serialized JSON
}

#[launch]
fn rocket() -> _ {
    println!("Master key: {}", *MASTER_KEY);
    
    let (tx, _) = channel::<String>(1024);

    rocket::build()
        .manage(EventChannels { captured_reqs: tx })
        //.mount("/front/", FileServer::from(relative!("../frontend/dist")))
        .mount("/", routes())
        .register("/", catchers())
}
