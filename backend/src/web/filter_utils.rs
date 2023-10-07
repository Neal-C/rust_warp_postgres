// Filter Utils

use std::{convert::Infallible, sync::Arc};

use warp::{reject::Rejection as WarpRejection, Filter as WarpFilter};

use crate::{
    model,
    security::{user_context_from_token, UserContext},
};

pub fn with_db(
    database: Arc<model::PostgresDatabase>,
) -> impl WarpFilter<Extract = (Arc<model::PostgresDatabase>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&database))
}

pub fn do_auth(
    _database: Arc<model::PostgresDatabase>,
) -> impl WarpFilter<Extract = (UserContext,), Error = warp::Rejection> + Clone {
    warp::any().and_then(|| async {
        Ok::<UserContext, WarpRejection>(user_context_from_token("123").await.unwrap())
    })
}
