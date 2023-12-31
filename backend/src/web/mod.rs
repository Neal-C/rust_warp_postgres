use std::{convert::Infallible, path::Path, sync::Arc};

use warp::Filter;
use warp::{reject::Rejection as WarpRejection, reply::Reply as WarpReply};

use crate::{model, security};
mod filter_utils;
pub use filter_utils::HEADER_XAUTH;
mod todo;

pub async fn start_web(
    web_folder: &str,
    web_port: u16,
    database: Arc<model::PostgresDatabase>,
) -> Result<(), Error> {
    // validate the web folder
    if !Path::new(web_folder).exists() {
        return Err(Error::FailStartWebFolderNotFound(web_folder.to_string()));
    }

    // Static content
    let content = warp::fs::dir(web_folder.to_string());
    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{web_folder}/index.html")));

    let static_site = content.or(root_index);

    // Combine all routes
    let routes = static_site.recover(handle_rejection);

    println!("Start 127.0.0.1:{web_port} at {web_folder}");
    warp::serve(routes).run(([127, 0, 0, 1], web_port)).await;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found")]
    FailStartWebFolderNotFound(String),

    #[error("Fail authentication missing X-Auth-Token header.")]
    FailAuthMissingXAuth,
}

// Warp Custom Message
#[derive(Debug)]
pub struct WebErrorMessage {
    pub typ: &'static str,
    pub message: String,
}

impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn rejection(typ: &'static str, message: String) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage { typ, message })
    }
}

impl From<self::Error> for warp::Rejection {
    fn from(other: self::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{other}"))
    }
}
impl From<model::Error> for warp::Rejection {
    fn from(other: model::Error) -> Self {
        WebErrorMessage::rejection("model::Error", format!("{other}"))
    }
}
impl From<security::Error> for warp::Rejection {
    fn from(other: security::Error) -> Self {
        WebErrorMessage::rejection("security::Error", format!("{other}"))
    }
}

async fn handle_rejection(err: WarpRejection) -> Result<impl WarpReply, Infallible> {
    //Print to server side

    //Call log API for capture and store

    //Build user message

    let user_message: String = match err.find::<WebErrorMessage>() {
        Some(err) => String::from(err.typ),
        None => String::from("Unknown error"),
    };

    let result: serde_json::Value = serde_json::json!({"{errorMessage": user_message});

    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(
        result,
        warp::http::StatusCode::BAD_REQUEST,
    ))
}
