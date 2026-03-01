#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::tokio::sync::broadcast::{channel, Sender};

pub mod routes;
pub mod models;
pub mod dyn_content;
pub mod capture;

use crate::routes::{routes, catchers};

pub const MASTER_KEY: &'static str = "changeme";  // TODO: randomize and print on start

pub struct EventChannels {
    // Only one for now
    captured_reqs: Sender<String>, // Serialized JSON
}

#[launch]
fn rocket() -> _ {
    
    let (tx, _) = channel::<String>(1024);

    rocket::build()
        .manage(EventChannels { captured_reqs: tx })
        .mount("/front/", FileServer::from(relative!("../frontend/dist")))
        .mount("/", routes())
        .register("/", catchers())
}
