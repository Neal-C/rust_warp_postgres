// #![allow(unused)]
// #![allow(unused_parens)]
// #![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::needless_return)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{env, sync::Arc};

use web::start_web;

mod model;
mod security;
mod web;

const DEFAULT_WEB_FOLDER: &str = "web-folder/";
const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    // Compute the web_folder
    let mut args: Vec<String> = env::args().collect();
    let web_folder: String = args.pop().unwrap_or_else(|| DEFAULT_WEB_FOLDER.to_string());
    let web_port = DEFAULT_WEB_PORT;

    // Get the database
    // In Production, the database might not be accessible right away, we should loop within a time range until accessible or too long to wait
    let database = model::initialize_database()
        .await
        .expect("Couldn't initialize the database");
    let database = Arc::new(database);

    // Start the server
    match start_web(&web_folder, web_port, database).await {
        Ok(_) => println!("Server ended"),
        Err(error) => println!("ERROR  - web server failed to start. Cause: {error:?}"),
    }
}
