#[macro_use] extern crate rocket;

use std::{sync::LazyLock};
use rocket::{fs::FileServer, tokio::sync::broadcast::{Sender, channel}};
use random_string::generate;
use systemd_journal_logger::JournalLog;
use log::{info, LevelFilter};
use std::env;


pub mod routes;
pub mod models;
pub mod dyn_content;
pub mod capture;

use crate::routes::{routes, catchers};

// TODO use env + secret.
pub static MASTER_KEY: LazyLock<String> = LazyLock::new(|| {
    generate(40, "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890")
});

pub static FRONT_PATH: LazyLock<Option<String>> = LazyLock::new(|| {
    env::var("FRONT_PATH").and_then(|s| { Ok(Some(s)) }).unwrap_or(None)
});

pub struct EventChannels {
    // Only one for now
    captured_reqs: Sender<String>, // Serialized JSON
}

#[launch]
fn rocket() -> _ {
    // journal logger init

    let env = env::var("ENV").unwrap_or("PROD".to_string());

    match &*env {
        "PROD" => {
            JournalLog::new().unwrap().install().unwrap();
            log::set_max_level(LevelFilter::Info);
            // TODO prod log mode
        },
        _ => {
            env_logger::init();
            // TODO debug log mode
        },
    }


    info!("Master key: {}", *MASTER_KEY);
    
    let (tx, _) = channel::<String>(1024);

    let mut r = rocket::build()
        .manage(EventChannels { captured_reqs: tx })
        .mount("/", routes())
        .register("/", catchers());

    if let Some(path) = &*FRONT_PATH {
        debug!("Front Path: {}", path);
        r = r.mount("/front/", FileServer::from(path));
    }

    r
}
