// Filter Utils

use std::{convert::Infallible, sync::Arc};

use warp::{reject::Rejection as WarpRejection, Filter as WarpFilter};

use crate::{
    model,
    security::{user_context_from_token, UserContext},
    web::Error as WebError,
};

pub const HEADER_XAUTH: &str = "X-AUTH-TOKEN";

pub fn with_db(
    database: Arc<model::PostgresDatabase>,
) -> impl WarpFilter<Extract = (Arc<model::PostgresDatabase>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&database))
}

pub fn do_auth(
    database: Arc<model::PostgresDatabase>,
) -> impl WarpFilter<Extract = (UserContext,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(with_db(database))
        .and(warp::header::optional(HEADER_XAUTH))
        .and_then(
            |database: Arc<model::PostgresDatabase>, xauth_token: Option<String>| async move {
                // async move because async closures are not supported yet, and 'move' because we're taking ownership of stuff
                // We'll also need explicit generics to help the compiler, as because of above reasons, the return type can't be infered
                match xauth_token {
                    Some(xauth) => {
                        // the &database is cast as the right thing because of the AsRef trait, so Arc<PostgresDatabase> = &PostgresDatabase
                        let user_ctx = user_context_from_token(&database, &xauth).await?;

                        Ok::<UserContext, WarpRejection>(user_ctx)
                    }
                    None => Err(WebError::FailAuthMissingXAuth.into()),
                }
            },
        )
}
